import { expect, test } from '@playwright/test';
test('preview lifecycle is explicit, usable and responsive', async ({
  page,
}) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));
  await page.goto('/');
  await expect(
    page.getByRole('button', { name: 'Continue with Google' }),
  ).toBeVisible();
  await expect(page.getByRole('button', { name: 'Start session' })).toHaveCount(
    0,
  );
  await page.getByRole('button', { name: 'Continue with Google' }).click();
  await expect(page.getByText('Preview — simulated runtime')).toBeVisible();
  await expect(page.locator('.app-header-kicker')).toHaveText(
    'Northstar Robotics',
  );
  await page.getByRole('button', { name: 'Start session' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByRole('button', { name: 'Check connection' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByRole('button', { name: 'Stop', exact: true }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime stopped.',
  );
  await page.getByRole('button', { name: 'Restart runtime' }).click();
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime is running.',
  );
  await page.getByLabel('Development profile').selectOption('student-a');
  await expect(page.getByRole('status')).toHaveText(
    'Simulated runtime stopped.',
  );
  await page.screenshot({ path: '.local/ui-preview.png', fullPage: true });
  await page.setViewportSize({ width: 390, height: 844 });
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
  expect(errors).toEqual([]);
});

test('workspace holding state reveals no product and can be retried', async ({
  page,
}) => {
  await page.goto('/?auth=membershipRequired');

  await expect(page.getByText('ada@example.com')).toBeVisible();
  await expect(page.getByRole('button', { name: 'Start session' })).toHaveCount(
    0,
  );
  await page.getByRole('button', { name: 'Check for access' }).click();
  await expect(page.locator('.app-header-kicker')).toHaveText(
    'Northstar Robotics',
  );
});

test('sign out removes the protected product without a reload', async ({
  page,
}) => {
  await page.goto('/?auth=authenticated');

  await expect(
    page.getByRole('button', { name: 'Start session' }),
  ).toBeVisible();
  await page.getByRole('button', { name: 'Settings' }).click();
  await page.getByRole('button', { name: 'Sign out' }).click();
  await expect(
    page.getByRole('button', { name: 'Continue with Google' }),
  ).toBeVisible();
  await expect(page.getByRole('button', { name: 'Start session' })).toHaveCount(
    0,
  );
});

test('workspace owner can pre-add exact Google email access', async ({
  page,
}) => {
  await page.goto('/?auth=authenticated&workspaceRole=owner');
  await page.getByRole('button', { name: 'Team' }).click();

  await expect(
    page.getByRole('heading', { name: 'People with access' }),
  ).toBeVisible();
  await page.getByLabel('Google email').fill('student+robotics@example.com');
  await page.getByLabel('Role').selectOption('student');
  await page.getByRole('button', { name: 'Add access' }).click();

  await expect(page.getByText('student+robotics@example.com')).toBeVisible();
  await expect(page.getByText('pending', { exact: true })).toBeVisible();
  await expect(page.getByText(/^Access added\./)).toBeVisible();
});

test('signed-out threshold does not overflow a narrow viewport', async ({
  page,
}) => {
  await page.setViewportSize({ width: 390, height: 844 });
  await page.goto('/');

  await expect(page.getByRole('heading', { name: /Begin with/ })).toBeVisible();
  expect(
    await page.evaluate(
      () => document.documentElement.scrollWidth <= window.innerWidth,
    ),
  ).toBe(true);
});

const authLayoutCases = [
  {
    scenario: 'signedOut',
    marker: /Begin with/,
    primaryAction: 'Continue with Google',
  },
  {
    scenario: 'membershipRequired',
    marker: /Your place is/,
    primaryAction: 'Check for access',
  },
  {
    scenario: 'offline',
    marker: /We lost the/,
    primaryAction: 'Try again',
  },
  {
    scenario: 'authenticated',
    marker: 'Northstar Robotics',
    primaryAction: 'Start session',
  },
] as const;

test('auth layouts remain within narrow and desktop viewports', async ({
  page,
}) => {
  const errors: string[] = [];
  page.on('pageerror', (error) => errors.push(error.message));

  for (const viewport of [
    { name: 'mobile', width: 390, height: 844 },
    { name: 'desktop', width: 1280, height: 900 },
  ] as const) {
    await page.setViewportSize(viewport);

    for (const authCase of authLayoutCases) {
      await page.goto(`/?auth=${authCase.scenario}`);
      const marker =
        authCase.scenario === 'authenticated'
          ? page.locator('.app-header-kicker')
          : page.getByText(authCase.marker).first();
      await expect(marker).toBeVisible();
      const primaryAction = page.getByRole('button', {
        name: authCase.primaryAction,
      });
      await expect(primaryAction).toBeVisible();
      const actionBox = await primaryAction.boundingBox();
      expect(actionBox).not.toBeNull();
      expect(actionBox!.y + actionBox!.height).toBeLessThanOrEqual(
        viewport.height,
      );
      expect(
        await page.evaluate(
          () => document.documentElement.scrollWidth <= window.innerWidth,
        ),
      ).toBe(true);
      await page.screenshot({
        path: `.local/auth-${authCase.scenario}-${viewport.name}.png`,
        fullPage: true,
      });
    }
  }

  expect(errors).toEqual([]);
});
