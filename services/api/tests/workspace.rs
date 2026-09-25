use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, Set, TransactionTrait,
};
use time::OffsetDateTime;
use tro_api::{
    auth::WorkspaceMembershipStore,
    entities::{account, workspace, workspace_audit_event, workspace_membership},
    workspace::WorkspaceService,
};
use uuid::Uuid;

#[tokio::test]
#[ignore = "requires the isolated PostgreSQL fixture; npm run test:integration"]
async fn owner_add_claim_remove_is_exact_idempotent_and_audited() {
    let fixture = tro_api::config::Config::from_env().unwrap();
    let database = tro_api::persistence::connect(&fixture.database_url)
        .await
        .unwrap();
    tro_api_migration::migrate(&database).await.unwrap();

    let owner_id = Uuid::new_v4();
    let learner_id = Uuid::new_v4();
    let workspace_id = Uuid::new_v4();
    let owner_membership_id = Uuid::new_v4();
    let now = OffsetDateTime::now_utc();
    for (id, display_name, email) in [
        (
            owner_id,
            "Workspace Owner",
            format!("{owner_id}@example.com"),
        ),
        (
            learner_id,
            "Robotics Learner",
            "student.name+robotics@gmail.com".to_owned(),
        ),
    ] {
        account::ActiveModel {
            id: Set(id),
            display_name: Set(display_name.to_owned()),
            verified_email: Set(email.clone()),
            email_normalized: Set(email.to_lowercase()),
            status: Set("active".to_owned()),
            created_at: Set(now),
            updated_at: Set(now),
        }
        .insert(&database)
        .await
        .unwrap();
    }
    workspace::ActiveModel {
        id: Set(workspace_id),
        name: Set("Northstar Robotics".to_owned()),
        status: Set("active".to_owned()),
        created_by_account_id: Set(owner_id),
        created_at: Set(now),
        updated_at: Set(now),
    }
    .insert(&database)
    .await
    .unwrap();
    workspace_membership::ActiveModel {
        id: Set(owner_membership_id),
        workspace_id: Set(workspace_id),
        account_id: Set(Some(owner_id)),
        email: Set(format!("{owner_id}@example.com")),
        email_normalized: Set(format!("{owner_id}@example.com")),
        role: Set("owner".to_owned()),
        added_by_account_id: Set(owner_id),
        created_at: Set(now),
        joined_at: Set(Some(now)),
        removed_at: Set(None),
    }
    .insert(&database)
    .await
    .unwrap();

    let service = WorkspaceService::new(database.clone());
    let correlation = Uuid::new_v4();
    let pending = service
        .add_member(
            owner_id,
            workspace_id,
            " Student.Name+Robotics@GMAIL.com ",
            "student",
            correlation,
        )
        .await
        .unwrap();
    assert_eq!(pending.state, "pending");
    assert_eq!(pending.role, "student");
    assert_eq!(pending.email, "Student.Name+Robotics@GMAIL.com");

    let repeated = service
        .add_member(
            owner_id,
            workspace_id,
            "student.name+robotics@gmail.com",
            "student",
            Uuid::new_v4(),
        )
        .await
        .unwrap();
    assert_eq!(repeated.membership_id, pending.membership_id);

    let transaction = database.begin().await.unwrap();
    let claimed = service
        .claim_pending_memberships(&transaction, learner_id, "student.name+robotics@gmail.com")
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    assert_eq!(claimed.len(), 1);
    assert_eq!(claimed[0].workspace_id, workspace_id);
    assert_eq!(claimed[0].role, "student");

    let transaction = database.begin().await.unwrap();
    let repeated_claim = service
        .claim_pending_memberships(&transaction, learner_id, "student.name+robotics@gmail.com")
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    assert_eq!(repeated_claim.len(), 1);

    let list = service
        .list_members(owner_id, workspace_id, Uuid::new_v4())
        .await
        .unwrap();
    assert_eq!(list.workspace.role, "owner");
    assert_eq!(list.members.len(), 2);
    let learner = list
        .members
        .iter()
        .find(|member| member.membership_id == pending.membership_id)
        .unwrap();
    assert_eq!(learner.state, "active");
    assert_eq!(learner.display_name.as_deref(), Some("Robotics Learner"));
    assert!(learner.joined_at.is_some());

    let audit_count = workspace_audit_event::Entity::find()
        .filter(workspace_audit_event::Column::WorkspaceId.eq(workspace_id))
        .count(&database)
        .await
        .unwrap();
    assert_eq!(audit_count, 2, "one add and one idempotent claim");

    service
        .remove_member(
            owner_id,
            workspace_id,
            pending.membership_id,
            Uuid::new_v4(),
        )
        .await
        .unwrap();
    let transaction = database.begin().await.unwrap();
    let active = service
        .active_memberships(&transaction, learner_id)
        .await
        .unwrap();
    transaction.commit().await.unwrap();
    assert!(active.is_empty());

    workspace_audit_event::Entity::delete_many()
        .filter(workspace_audit_event::Column::WorkspaceId.eq(workspace_id))
        .exec(&database)
        .await
        .unwrap();
    workspace_membership::Entity::delete_many()
        .filter(workspace_membership::Column::WorkspaceId.eq(workspace_id))
        .exec(&database)
        .await
        .unwrap();
    workspace::Entity::delete_by_id(workspace_id)
        .exec(&database)
        .await
        .unwrap();
    account::Entity::delete_many()
        .filter(account::Column::Id.is_in([owner_id, learner_id]))
        .exec(&database)
        .await
        .unwrap();
}
