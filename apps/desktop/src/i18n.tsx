import {
  createContext,
  useCallback,
  useContext,
  useEffect,
  useMemo,
  useState,
  type ReactNode,
} from 'react';

export type AppLocale = 'en' | 'vi';

const STORAGE_KEY = 'tro.app-locale';

const english = {
  'language.label': 'App language',
  'language.english': 'English',
  'language.vietnamese': 'Tiếng Việt',
  'language.description':
    'Changes menus, buttons, and system messages. It does not change voice transcription or the language of a lesson.',
  'nav.workspaceNavigation': 'Workspace navigation',
  'nav.workspace': 'Workspace',
  'nav.learning': 'Learning',
  'nav.workspaceTools': 'Workspace tools',
  'nav.learn': 'Learn',
  'nav.team': 'Team',
  'nav.settings': 'Settings',
  'common.privateByDesign': 'Private by design',
  'common.previewRuntime': 'Preview — simulated runtime',
  'common.presentationStack':
    'React presentation · Rust supervision · Python runtime',
  'common.pleaseWait': 'Please wait…',
  'common.signOut': 'Sign out',
  'common.signingOut': 'Signing out…',
  'role.owner': 'Owner',
  'role.teacher': 'Teacher',
  'role.student': 'Student',
  'member.active': 'Active',
  'member.pending': 'Pending',
  'app.learn.eyebrow': 'A solid place to begin',
  'app.learn.title': 'Make room\nfor learning.',
  'app.learn.description':
    'One desktop. One private runtime.\nA clear foundation for what comes next.',
  'app.team.eyebrow': 'Workspace access',
  'app.team.title': 'Your workspace, your people.',
  'app.team.description':
    'Keep the roster intentional. Access begins only after Google verifies the exact email you add here.',
  'settings.eyebrow': 'Account, voice & access',
  'settings.title': 'Settings',
  'settings.description':
    'Your profile, voice preference, and secure device session for this workspace.',
  'settings.profile': 'Profile',
  'settings.workspace': 'Workspace',
  'settings.secureSession': 'Secure device session',
  'settings.secureSessionDescription':
    'Your sign-in stays on this device and is checked against current workspace access.',
  'voiceLanguage.category': 'Voice',
  'voiceLanguage.label': 'Transcription language',
  'voiceLanguage.description':
    'Choose the language you expect to speak. English and Vietnamese focus recognition; Auto detects the input language. Changes apply to your next voice instruction and do not translate it.',
  'voiceLanguage.vietnamese': 'Vietnamese',
  'voiceLanguage.english': 'English',
  'voiceLanguage.auto': 'Auto',
  'voiceLanguage.saving': 'Saving…',
  'voiceLanguage.unavailable':
    'Transcription language is unavailable. Try again.',
  'voiceLanguage.saveError':
    'Transcription language could not be saved. Try again.',
  'onboarding.boundaryError':
    'Tro could not verify device readiness. Check your connection to the desktop host and try again.',
  'onboarding.eyebrow': 'Device checkup',
  'onboarding.unavailable': 'Device check unavailable',
  'onboarding.checking': 'Checking this device…',
  'onboarding.noPrompt': 'No permission prompt will appear.',
  'onboarding.tryAgain': 'Try again',
  'onboarding.checkFailed':
    'The device check did not finish. Your previous status is unchanged.',
  'onboarding.openSettingsFailed':
    'Tro could not open device settings. Open the page manually, then recheck.',
  'onboarding.relaunchFailed':
    'Tro could not relaunch. Close and reopen the app, then recheck.',
  'onboarding.checkingPermissions': 'Checking device permissions…',
  'onboarding.privateSetup': 'Private device setup',
  'onboarding.permissions': 'Device permissions',
  'onboarding.setup': 'Set up this device',
  'onboarding.description':
    'Tro observes only the window you choose. It does not save screen video, and this check retains no audio.',
  'onboarding.screenControls': 'Screen & controls',
  'onboarding.required': 'Required',
  'onboarding.screenCapture': 'Screen capture',
  'onboarding.accessibility': 'Accessibility',
  'onboarding.windowsPicker':
    'Windows uses its secure window picker when you start Observe. There is no app-list setting to change here.',
  'onboarding.nextAccessibility': 'Next, let’s add Tro to Accessibility.',
  'onboarding.openScreenRecording':
    'Let’s open the exact Screen Recording page.',
  'onboarding.rechecking': 'Rechecking…',
  'onboarding.openingSettings': 'Opening settings…',
  'onboarding.changedRecheck': 'I changed it — recheck',
  'onboarding.openSettings': 'Open {target} settings',
  'onboarding.recheckScreen': 'Recheck screen access',
  'onboarding.microphone': 'Microphone',
  'onboarding.optional': 'Optional',
  'onboarding.microphoneNote':
    'This prepares optional push-to-talk. Text always works, and the probe stores no samples.',
  'onboarding.microphoneChoice':
    'You can check your microphone here, or keep using text.',
  'onboarding.checkingMicrophone': 'Checking…',
  'onboarding.checkMicrophone': 'Check microphone',
  'onboarding.openMicrophone': 'Open the exact microphone privacy page.',
  'onboarding.useText': 'Use text instead',
  'onboarding.ready': 'Ready to learn',
  'onboarding.relaunchNeeded': 'Relaunch needed',
  'onboarding.finalStep': 'Final step',
  'onboarding.relaunch': 'Relaunch Tro',
  'onboarding.continue': 'Continue to Learn',
  'onboarding.statusFallback': 'Device permission status updated.',
  'onboarding.recheckDevice': 'Recheck device',
  'onboarding.settingsOpen': '{title} settings are open',
  'onboarding.pointerBoundary':
    'Tro’s floating guide only points. You make every change.',
  'onboarding.status.granted': 'Ready',
  'onboarding.status.available': 'Available',
  'onboarding.status.notDetermined': 'Not checked',
  'onboarding.status.denied': 'Needs attention',
  'onboarding.status.unavailable': 'Unavailable',
  'onboarding.status.unknown': 'Not confirmed',
  'onboarding.capabilityFallback': 'Review this permission before continuing.',
  'permissionGuide.aria': 'Tro permission guide',
  'permissionGuide.kicker': 'Look in System Settings',
  'permissionGuide.target': 'Find Tro in this list',
  'permissionGuide.boundary':
    'You make every change. Tro only points out where.',
  'permissionGuide.windowsMicrophone.title': 'Microphone access',
  'permissionGuide.windowsMicrophone.path': 'Privacy & security → Microphone',
  'permissionGuide.windowsMicrophone.instruction':
    'Turn on Microphone access, then turn on “Let desktop apps access your microphone.” Windows uses this shared switch for Tro.',
  'permissionGuide.accessibility.title': 'Accessibility',
  'permissionGuide.accessibility.path': 'Privacy & Security → Accessibility',
  'permissionGuide.accessibility.instruction':
    'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open. Drag Tro.app from Applications into the app list also works.',
  'permissionGuide.microphone.title': 'Microphone',
  'permissionGuide.microphone.path': 'Privacy & Security → Microphone',
  'permissionGuide.microphone.instruction':
    'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open.',
  'permissionGuide.screenCapture.title': 'Screen Recording',
  'permissionGuide.screenCapture.path':
    'Privacy & Security → Screen & System Audio Recording',
  'permissionGuide.screenCapture.instruction':
    'Find Tro and turn it on. If Tro is missing, click +, then choose Applications → Tro.app → Open. Drag Tro.app from Applications into the app list also works.',
  'auth.secureSession': 'Secure device session',
  'auth.signedInAccount': 'Signed-in Google account',
  'auth.preview': 'Preview · simulated sign-in',
  'auth.ready': 'Ready when you are',
  'auth.progress': 'Sign-in progress',
  'auth.pathCaption': 'A path into the lesson',
  'auth.identify': 'Identify',
  'auth.trustedAccount': 'Your trusted account',
  'auth.findStudio': 'Find your studio',
  'auth.waitingAccess': 'Waiting for access',
  'auth.sharedWorkspace': 'Your shared workspace',
  'auth.keepLearning': 'Keep learning',
  'auth.returnToWork': 'Return to your work',
  'auth.checking.eyebrow': 'Finding your place',
  'auth.checking.title': 'Picking up\nwhere you left off.',
  'auth.checking.note':
    'Tro is checking the secure session kept by this device.',
  'auth.signedOut.eyebrow': 'Your learning space',
  'auth.signedOut.title': 'Begin with\nwho you are.',
  'auth.signedOut.note':
    'Use the Google account your workspace owner added to Tro.',
  'auth.signedOut.action': 'Continue with Google',
  'auth.signingIn.eyebrow': 'One small detour',
  'auth.signingIn.title': 'Your browser has\nthe next step.',
  'auth.signingIn.note':
    'Finish signing in there. This window will continue on its own.',
  'auth.membershipRequired.eyebrow': 'Account confirmed',
  'auth.membershipRequired.title': 'Your place is\nalmost ready.',
  'auth.membershipRequired.note':
    'Ask a workspace owner to add this exact Google email, then try again.',
  'auth.membershipRequired.action': 'Check for access',
  'auth.offline.eyebrow': 'Path interrupted',
  'auth.offline.title': 'We lost the\nconnection.',
  'auth.offline.note':
    'Your secure device session is still here. Reconnect and try again.',
  'auth.offline.action': 'Try again',
  'auth.error.eyebrow': 'Sign-in needs attention',
  'auth.error.title': 'This path is not\nready just yet.',
  'auth.error.note':
    'Nothing private was shared. Ask your Tro administrator if this continues.',
  'auth.error.action': 'Try again',
  'auth.boundaryError':
    'Tro could not verify the secure session response. Try again in a moment.',
  'auth.notConfigured': 'Google sign-in is not configured for this build.',
  'auth.status.checking': 'Checking this device for a secure session…',
  'auth.status.signedOut': 'Sign in to continue to your Tro workspace.',
  'auth.status.signingIn': 'Waiting for Google sign-in in your browser…',
  'auth.status.authenticated': 'Signed in securely.',
  'auth.status.membershipRequired':
    'This account is not in an active workspace yet.',
  'auth.status.offline': 'Tro cannot reach the sign-in service right now.',
  'auth.status.error': 'Sign-in could not be completed. Try again.',
  'runtime.heading': 'Private teaching runtime',
  'runtime.start': 'Start session',
  'runtime.check': 'Check connection',
  'runtime.stop': 'Stop',
  'runtime.restart': 'Restart runtime',
  'runtime.profile': 'Development profile',
  'runtime.diagnosticOnly': 'Diagnostic only',
  'runtime.studentA': 'Student A',
  'runtime.studentB': 'Student B',
  'runtime.finePrint':
    'A diagnostic session checks the process connection. Teaching guidance observes a selected window and never performs computer input.',
  'runtime.connectError': 'Unable to connect to runtime status.',
  'runtime.requestError':
    'The request failed. Check the local runtime and retry.',
  'runtime.unauthorized': 'The development profile could not be authenticated.',
  'runtime.notReady': 'Runtime unavailable. Check setup and the local API.',
  'runtime.timeout': 'The runtime timed out. Stop it, then retry.',
  'runtime.protocolMismatch': 'Runtime versions differ. Run setup again.',
  'runtime.state.stopped': 'Stopped',
  'runtime.state.starting': 'Starting',
  'runtime.state.running': 'Running',
  'runtime.state.stopping': 'Stopping',
  'runtime.state.failed': 'Failed',
  'runtime.status.fallback': 'The private runtime status changed.',
  'teaching.aria': 'Visual teaching',
  'teaching.heading': 'See where. Try it yourself.',
  'teaching.permissions': 'Observation permissions',
  'teaching.description':
    'Tro shows visual guidance. You perform every click, drag, keystroke and scroll.',
  'teaching.connectProof': 'Connect proof account',
  'teaching.question': 'What would you like help with?',
  'teaching.plan': 'Show me how / show another way',
  'teaching.findWindows': 'Find windows',
  'teaching.stop': 'Stop guidance',
  'teaching.practiceWindow': 'Practice window',
  'teaching.selectWindow': 'Select a window you opened',
  'teaching.untitledWindow': 'Untitled window',
  'teaching.observe': 'Observe',
  'teaching.observation': 'Observation',
  'teaching.screenshot': 'Screenshot',
  'teaching.accessibility': 'Accessibility',
  'teaching.model': 'Model',
  'teaching.connectModel': 'Connect your proof account first.',
  'teaching.observeAgain': 'Check observation access, then observe again.',
  'teaching.retryPlan':
    'Model access or planning failed. Reconnect if your grant expired, then explicitly replan. Previous progress is preserved.',
  'teaching.readiness.unknown': 'Unknown',
  'teaching.readiness.available': 'Available',
  'teaching.readiness.unavailable': 'Unavailable',
  'teaching.readiness.unconfigured': 'Not configured',
  'teaching.readiness.ready': 'Ready',
  'teaching.planAria': 'Teaching plan',
  'teaching.steps': 'Your steps',
  'teaching.resume': 'Resume guidance',
  'teaching.pause': 'Pause guidance',
  'teaching.continue': 'Continue — I’m ready',
  'teaching.manualTools': 'Manual guidance tools',
  'teaching.visualGuidance': 'Visual guidance',
  'teaching.control': 'Control',
  'teaching.chooseControl': 'Choose an observed control',
  'teaching.gesture': 'Gesture',
  'teaching.destination': 'Destination',
  'teaching.chooseDestination': 'Choose destination',
  'teaching.direction': 'Direction',
  'teaching.caption': 'Caption',
  'teaching.guidanceLanguage': 'Guidance language',
  'teaching.guidanceLanguageNote':
    'This controls lesson guidance only. App language and voice transcription stay separate.',
  'teaching.showGuidance': 'Show / repeat guidance',
  'teaching.expectedLabel': 'Expected control label',
  'teaching.expectedValue': 'Expected value',
  'teaching.check': 'I tried it — check',
  'teaching.outcome.confirmed': 'Confirmed',
  'teaching.outcome.mismatch': 'Not matched',
  'teaching.outcome.unknown': 'Unknown',
  'teaching.liveUnavailable': 'Live guidance updates are unavailable.',
  'teaching.permissionUnavailable': 'Permission check unavailable.',
  'teaching.permissionHelp':
    'Enable screen observation and accessibility permissions, then try again.',
  'teaching.proofUnavailable':
    'Configure a private proof account on this device first.',
  'teaching.unavailable':
    'Guidance is unavailable. Start the runtime, check permissions, and observe again.',
  'teaching.overlayAria': '{gesture} guidance: {caption}',
  'teaching.overlayStep': 'Step {current} of {total} · {gesture}',
  'gesture.point': 'Point',
  'gesture.click': 'Click',
  'gesture.drag': 'Drag',
  'gesture.type': 'Type',
  'gesture.scroll': 'Scroll',
  'gesture.action.point': 'Look here',
  'gesture.action.click': 'Click',
  'gesture.action.drag': 'Drag',
  'gesture.action.type': 'Type',
  'gesture.action.scroll': 'Scroll',
  'direction.up': 'Up',
  'direction.down': 'Down',
  'direction.left': 'Left',
  'direction.right': 'Right',
  'voice.loading': 'Loading voice guidance…',
  'voice.aria': 'Voice guidance',
  'voice.eyebrow': 'Ask Tro to show you',
  'voice.heading': 'Hold two keys. Ask. Release.',
  'voice.enable': 'Enable voice',
  'voice.permissions': 'Check permissions',
  'voice.disable': 'Disable voice',
  'voice.cancel': 'Cancel guidance',
  'voice.textLabel': 'Type instead of speaking',
  'voice.textPlaceholder': 'e.g. Show me how to open the settings panel',
  'voice.run': 'Show me how',
  'voice.finalInstruction': 'Your request',
  'voice.listeningTranscript': 'Listening transcript',
  'voice.clearTranscript': 'Clear transcript',
  'voice.selectedWindow': 'Selected window:',
  'voice.statusUnavailable': 'Voice status is unavailable.',
  'voice.statusUnverified': 'Voice status could not be verified.',
  'voice.requestFailed': 'Voice guidance could not complete that request.',
  'voice.permissionRecovery':
    'Allow microphone and keyboard monitoring access, then check permissions again.',
  'voice.phase.disabled': 'Disabled',
  'voice.phase.idle': 'Ready',
  'voice.phase.listening': 'Listening',
  'voice.phase.transcribing': 'Transcribing',
  'voice.phase.dispatching': 'Preparing',
  'voice.phase.planning': 'Planning',
  'voice.phase.guiding': 'Guiding',
  'voice.phase.completed': 'Completed',
  'voice.phase.cancelled': 'Cancelled',
  'voice.phase.failed': 'Failed',
  'voice.status.fallback': 'Voice guidance status changed.',
  'workspace.eyebrow': 'Workspace authority',
  'workspace.heading': 'People with access',
  'workspace.assigned': '{count} assigned',
  'workspace.description':
    'Add the exact Google email. Tro connects it automatically after the account signs in—no invitation link required.',
  'workspace.email': 'Google email',
  'workspace.role': 'Role',
  'workspace.saving': 'Saving…',
  'workspace.add': 'Add access',
  'workspace.loading': 'Loading workspace members…',
  'workspace.confirmRemoval': 'Confirm removal',
  'workspace.remove': 'Remove access',
  'workspace.invalidEmail': 'Enter the exact Google email for this member.',
  'workspace.loadError': 'Workspace members could not be loaded. Try again.',
  'workspace.added':
    'Access added. Tro will connect this account after Google verifies that email.',
  'workspace.addError':
    'Access could not be added. Check the email and existing role.',
  'workspace.confirmMessage': 'Confirm removal for {email}.',
  'workspace.removed': 'Workspace access removed.',
  'workspace.removeError': 'Workspace access could not be removed. Try again.',
} as const;

export type TranslationKey = keyof typeof english;

const vietnamese: Record<TranslationKey, string> = {
  'language.label': 'Ngôn ngữ ứng dụng',
  'language.english': 'English',
  'language.vietnamese': 'Tiếng Việt',
  'language.description':
    'Thay đổi menu, nút và thông báo hệ thống. Không thay đổi ngôn ngữ nhận dạng giọng nói hoặc ngôn ngữ bài học.',
  'nav.workspaceNavigation': 'Điều hướng không gian làm việc',
  'nav.workspace': 'Không gian làm việc',
  'nav.learning': 'Học tập',
  'nav.workspaceTools': 'Công cụ không gian làm việc',
  'nav.learn': 'Học tập',
  'nav.team': 'Thành viên',
  'nav.settings': 'Cài đặt',
  'common.privateByDesign': 'Riêng tư ngay từ thiết kế',
  'common.previewRuntime': 'Bản xem trước — môi trường mô phỏng',
  'common.presentationStack':
    'Giao diện React · Điều phối Rust · Môi trường Python',
  'common.pleaseWait': 'Vui lòng chờ…',
  'common.signOut': 'Đăng xuất',
  'common.signingOut': 'Đang đăng xuất…',
  'role.owner': 'Chủ sở hữu',
  'role.teacher': 'Giáo viên',
  'role.student': 'Học sinh',
  'member.active': 'Đang hoạt động',
  'member.pending': 'Đang chờ',
  'app.learn.eyebrow': 'Nơi vững chắc để bắt đầu',
  'app.learn.title': 'Dành không gian\ncho việc học.',
  'app.learn.description':
    'Một máy tính. Một môi trường riêng tư.\nNền tảng rõ ràng cho những bước tiếp theo.',
  'app.team.eyebrow': 'Quyền truy cập không gian làm việc',
  'app.team.title': 'Không gian của bạn, thành viên của bạn.',
  'app.team.description':
    'Quản lý danh sách thành viên có chủ đích. Quyền truy cập chỉ bắt đầu sau khi Google xác minh đúng email bạn thêm tại đây.',
  'settings.eyebrow': 'Tài khoản, giọng nói và quyền truy cập',
  'settings.title': 'Cài đặt',
  'settings.description':
    'Hồ sơ, tùy chọn giọng nói và phiên thiết bị bảo mật của bạn trong không gian làm việc này.',
  'settings.profile': 'Hồ sơ',
  'settings.workspace': 'Không gian làm việc',
  'settings.secureSession': 'Phiên thiết bị bảo mật',
  'settings.secureSessionDescription':
    'Thông tin đăng nhập được giữ trên thiết bị này và được đối chiếu với quyền truy cập hiện tại.',
  'voiceLanguage.category': 'Giọng nói',
  'voiceLanguage.label': 'Ngôn ngữ nhận dạng giọng nói',
  'voiceLanguage.description':
    'Chọn ngôn ngữ bạn dự định nói. Tiếng Anh và tiếng Việt giúp tập trung nhận dạng; Tự động sẽ phát hiện ngôn ngữ đầu vào. Thay đổi áp dụng cho chỉ dẫn bằng giọng nói tiếp theo và không dịch nội dung đó.',
  'voiceLanguage.vietnamese': 'Tiếng Việt',
  'voiceLanguage.english': 'Tiếng Anh',
  'voiceLanguage.auto': 'Tự động',
  'voiceLanguage.saving': 'Đang lưu…',
  'voiceLanguage.unavailable':
    'Không thể tải ngôn ngữ nhận dạng giọng nói. Hãy thử lại.',
  'voiceLanguage.saveError':
    'Không thể lưu ngôn ngữ nhận dạng giọng nói. Hãy thử lại.',
  'onboarding.boundaryError':
    'Tro không thể xác minh trạng thái sẵn sàng của thiết bị. Hãy kiểm tra kết nối với ứng dụng máy tính rồi thử lại.',
  'onboarding.eyebrow': 'Kiểm tra thiết bị',
  'onboarding.unavailable': 'Không thể kiểm tra thiết bị',
  'onboarding.checking': 'Đang kiểm tra thiết bị này…',
  'onboarding.noPrompt': 'Sẽ không xuất hiện yêu cầu cấp quyền nào.',
  'onboarding.tryAgain': 'Thử lại',
  'onboarding.checkFailed':
    'Kiểm tra thiết bị chưa hoàn tất. Trạng thái trước đó của bạn không thay đổi.',
  'onboarding.openSettingsFailed':
    'Tro không thể mở cài đặt thiết bị. Hãy tự mở trang đó rồi kiểm tra lại.',
  'onboarding.relaunchFailed':
    'Tro không thể khởi động lại. Hãy đóng và mở lại ứng dụng rồi kiểm tra lại.',
  'onboarding.checkingPermissions': 'Đang kiểm tra quyền trên thiết bị…',
  'onboarding.privateSetup': 'Thiết lập thiết bị riêng tư',
  'onboarding.permissions': 'Quyền trên thiết bị',
  'onboarding.setup': 'Thiết lập thiết bị này',
  'onboarding.description':
    'Tro chỉ quan sát cửa sổ bạn chọn. Tro không lưu video màn hình và lần kiểm tra này không giữ lại âm thanh.',
  'onboarding.screenControls': 'Màn hình và điều khiển',
  'onboarding.required': 'Bắt buộc',
  'onboarding.screenCapture': 'Chụp màn hình',
  'onboarding.accessibility': 'Trợ năng',
  'onboarding.windowsPicker':
    'Windows dùng trình chọn cửa sổ bảo mật khi bạn bắt đầu Quan sát. Không có danh sách ứng dụng nào cần thay đổi tại đây.',
  'onboarding.nextAccessibility': 'Tiếp theo, hãy thêm Tro vào mục Trợ năng.',
  'onboarding.openScreenRecording': 'Hãy mở đúng trang Ghi màn hình.',
  'onboarding.rechecking': 'Đang kiểm tra lại…',
  'onboarding.openingSettings': 'Đang mở cài đặt…',
  'onboarding.changedRecheck': 'Tôi đã thay đổi — kiểm tra lại',
  'onboarding.openSettings': 'Mở cài đặt {target}',
  'onboarding.recheckScreen': 'Kiểm tra lại quyền màn hình',
  'onboarding.microphone': 'Micrô',
  'onboarding.optional': 'Tùy chọn',
  'onboarding.microphoneNote':
    'Bước này chuẩn bị cho tính năng nhấn để nói tùy chọn. Văn bản luôn dùng được và lần kiểm tra không lưu mẫu âm thanh.',
  'onboarding.microphoneChoice':
    'Bạn có thể kiểm tra micrô tại đây hoặc tiếp tục dùng văn bản.',
  'onboarding.checkingMicrophone': 'Đang kiểm tra…',
  'onboarding.checkMicrophone': 'Kiểm tra micrô',
  'onboarding.openMicrophone': 'Mở đúng trang quyền riêng tư của micrô.',
  'onboarding.useText': 'Dùng văn bản thay thế',
  'onboarding.ready': 'Sẵn sàng học',
  'onboarding.relaunchNeeded': 'Cần khởi động lại',
  'onboarding.finalStep': 'Bước cuối',
  'onboarding.relaunch': 'Khởi động lại Tro',
  'onboarding.continue': 'Tiếp tục đến phần Học tập',
  'onboarding.statusFallback': 'Trạng thái quyền thiết bị đã được cập nhật.',
  'onboarding.recheckDevice': 'Kiểm tra lại thiết bị',
  'onboarding.settingsOpen': 'Cài đặt {title} đang mở',
  'onboarding.pointerBoundary':
    'Hướng dẫn nổi của Tro chỉ trỏ vị trí. Bạn tự thực hiện mọi thay đổi.',
  'onboarding.status.granted': 'Sẵn sàng',
  'onboarding.status.available': 'Có sẵn',
  'onboarding.status.notDetermined': 'Chưa kiểm tra',
  'onboarding.status.denied': 'Cần xử lý',
  'onboarding.status.unavailable': 'Không có sẵn',
  'onboarding.status.unknown': 'Chưa xác nhận',
  'onboarding.capabilityFallback': 'Hãy xem lại quyền này trước khi tiếp tục.',
  'permissionGuide.aria': 'Hướng dẫn cấp quyền Tro',
  'permissionGuide.kicker': 'Tìm trong Cài đặt hệ thống',
  'permissionGuide.target': 'Tìm Tro trong danh sách này',
  'permissionGuide.boundary':
    'Bạn tự thực hiện mọi thay đổi. Tro chỉ chỉ ra vị trí.',
  'permissionGuide.windowsMicrophone.title': 'Quyền truy cập micrô',
  'permissionGuide.windowsMicrophone.path': 'Quyền riêng tư và bảo mật → Micrô',
  'permissionGuide.windowsMicrophone.instruction':
    'Bật Quyền truy cập micrô, sau đó bật “Cho phép ứng dụng máy tính truy cập micrô”. Windows dùng công tắc chung này cho Tro.',
  'permissionGuide.accessibility.title': 'Trợ năng',
  'permissionGuide.accessibility.path': 'Quyền riêng tư và bảo mật → Trợ năng',
  'permissionGuide.accessibility.instruction':
    'Tìm Tro và bật lên. Nếu không có Tro, nhấp + rồi chọn Ứng dụng → Tro.app → Mở. Bạn cũng có thể kéo Tro.app từ thư mục Ứng dụng vào danh sách.',
  'permissionGuide.microphone.title': 'Micrô',
  'permissionGuide.microphone.path': 'Quyền riêng tư và bảo mật → Micrô',
  'permissionGuide.microphone.instruction':
    'Tìm Tro và bật lên. Nếu không có Tro, nhấp + rồi chọn Ứng dụng → Tro.app → Mở.',
  'permissionGuide.screenCapture.title': 'Ghi màn hình',
  'permissionGuide.screenCapture.path':
    'Quyền riêng tư và bảo mật → Ghi màn hình và âm thanh hệ thống',
  'permissionGuide.screenCapture.instruction':
    'Tìm Tro và bật lên. Nếu không có Tro, nhấp + rồi chọn Ứng dụng → Tro.app → Mở. Bạn cũng có thể kéo Tro.app từ thư mục Ứng dụng vào danh sách.',
  'auth.secureSession': 'Phiên thiết bị bảo mật',
  'auth.signedInAccount': 'Tài khoản Google đã đăng nhập',
  'auth.preview': 'Bản xem trước · đăng nhập mô phỏng',
  'auth.ready': 'Sẵn sàng khi bạn sẵn sàng',
  'auth.progress': 'Tiến trình đăng nhập',
  'auth.pathCaption': 'Con đường vào bài học',
  'auth.identify': 'Xác định danh tính',
  'auth.trustedAccount': 'Tài khoản đáng tin cậy của bạn',
  'auth.findStudio': 'Tìm lớp học của bạn',
  'auth.waitingAccess': 'Đang chờ cấp quyền',
  'auth.sharedWorkspace': 'Không gian làm việc chung',
  'auth.keepLearning': 'Tiếp tục học',
  'auth.returnToWork': 'Quay lại bài đang làm',
  'auth.checking.eyebrow': 'Đang tìm đúng vị trí của bạn',
  'auth.checking.title': 'Tiếp tục\ntừ nơi bạn dừng lại.',
  'auth.checking.note':
    'Tro đang kiểm tra phiên bảo mật được lưu trên thiết bị này.',
  'auth.signedOut.eyebrow': 'Không gian học tập của bạn',
  'auth.signedOut.title': 'Bắt đầu bằng\ndanh tính của bạn.',
  'auth.signedOut.note':
    'Dùng tài khoản Google mà chủ sở hữu không gian làm việc đã thêm vào Tro.',
  'auth.signedOut.action': 'Tiếp tục với Google',
  'auth.signingIn.eyebrow': 'Một bước chuyển nhỏ',
  'auth.signingIn.title': 'Bước tiếp theo\nđang ở trình duyệt.',
  'auth.signingIn.note':
    'Hoàn tất đăng nhập tại đó. Cửa sổ này sẽ tự tiếp tục.',
  'auth.membershipRequired.eyebrow': 'Đã xác nhận tài khoản',
  'auth.membershipRequired.title': 'Vị trí của bạn\ngần sẵn sàng.',
  'auth.membershipRequired.note':
    'Hãy nhờ chủ sở hữu thêm đúng email Google này rồi thử lại.',
  'auth.membershipRequired.action': 'Kiểm tra quyền truy cập',
  'auth.offline.eyebrow': 'Kết nối bị gián đoạn',
  'auth.offline.title': 'Đã mất\nkết nối.',
  'auth.offline.note':
    'Phiên thiết bị bảo mật của bạn vẫn còn. Hãy kết nối lại và thử lại.',
  'auth.offline.action': 'Thử lại',
  'auth.error.eyebrow': 'Đăng nhập cần được xử lý',
  'auth.error.title': 'Đường dẫn này\nchưa sẵn sàng.',
  'auth.error.note':
    'Không có dữ liệu riêng tư nào được chia sẻ. Hãy hỏi quản trị viên Tro nếu lỗi tiếp diễn.',
  'auth.error.action': 'Thử lại',
  'auth.boundaryError':
    'Tro không thể xác minh phản hồi phiên bảo mật. Hãy thử lại sau ít phút.',
  'auth.notConfigured': 'Bản dựng này chưa cấu hình đăng nhập Google.',
  'auth.status.checking': 'Đang kiểm tra phiên bảo mật trên thiết bị này…',
  'auth.status.signedOut':
    'Hãy đăng nhập để tiếp tục vào không gian làm việc Tro.',
  'auth.status.signingIn': 'Đang chờ đăng nhập Google trong trình duyệt…',
  'auth.status.authenticated': 'Đã đăng nhập an toàn.',
  'auth.status.membershipRequired':
    'Tài khoản này chưa thuộc không gian làm việc đang hoạt động.',
  'auth.status.offline': 'Tro hiện không thể kết nối dịch vụ đăng nhập.',
  'auth.status.error': 'Không thể hoàn tất đăng nhập. Hãy thử lại.',
  'runtime.heading': 'Môi trường giảng dạy riêng tư',
  'runtime.start': 'Bắt đầu phiên',
  'runtime.check': 'Kiểm tra kết nối',
  'runtime.stop': 'Dừng',
  'runtime.restart': 'Khởi động lại môi trường',
  'runtime.profile': 'Hồ sơ phát triển',
  'runtime.diagnosticOnly': 'Chỉ chẩn đoán',
  'runtime.studentA': 'Học sinh A',
  'runtime.studentB': 'Học sinh B',
  'runtime.finePrint':
    'Phiên chẩn đoán kiểm tra kết nối tiến trình. Hướng dẫn của Tro chỉ quan sát cửa sổ đã chọn và không bao giờ thực hiện thao tác máy tính.',
  'runtime.connectError': 'Không thể kết nối trạng thái môi trường.',
  'runtime.requestError':
    'Yêu cầu thất bại. Hãy kiểm tra môi trường cục bộ rồi thử lại.',
  'runtime.unauthorized': 'Không thể xác thực hồ sơ phát triển.',
  'runtime.notReady':
    'Môi trường chưa sẵn sàng. Hãy kiểm tra thiết lập và API cục bộ.',
  'runtime.timeout': 'Môi trường đã hết thời gian chờ. Hãy dừng rồi thử lại.',
  'runtime.protocolMismatch':
    'Phiên bản môi trường không khớp. Hãy chạy lại thiết lập.',
  'runtime.state.stopped': 'Đã dừng',
  'runtime.state.starting': 'Đang khởi động',
  'runtime.state.running': 'Đang chạy',
  'runtime.state.stopping': 'Đang dừng',
  'runtime.state.failed': 'Thất bại',
  'runtime.status.fallback': 'Trạng thái môi trường riêng tư đã thay đổi.',
  'teaching.aria': 'Hướng dẫn trực quan',
  'teaching.heading': 'Nhìn đúng chỗ. Tự mình thử.',
  'teaching.permissions': 'Quyền quan sát',
  'teaching.description':
    'Tro hiển thị hướng dẫn trực quan. Bạn tự thực hiện mọi thao tác nhấp, kéo, gõ phím và cuộn.',
  'teaching.connectProof': 'Kết nối tài khoản thử nghiệm',
  'teaching.question': 'Bạn muốn được giúp việc gì?',
  'teaching.plan': 'Chỉ tôi cách làm / cách khác',
  'teaching.findWindows': 'Tìm cửa sổ',
  'teaching.stop': 'Dừng hướng dẫn',
  'teaching.practiceWindow': 'Cửa sổ thực hành',
  'teaching.selectWindow': 'Chọn một cửa sổ bạn đã mở',
  'teaching.untitledWindow': 'Cửa sổ không có tiêu đề',
  'teaching.observe': 'Quan sát',
  'teaching.observation': 'Quan sát',
  'teaching.screenshot': 'Ảnh chụp màn hình',
  'teaching.accessibility': 'Trợ năng',
  'teaching.model': 'Mô hình',
  'teaching.connectModel': 'Hãy kết nối tài khoản thử nghiệm trước.',
  'teaching.observeAgain': 'Hãy kiểm tra quyền quan sát rồi quan sát lại.',
  'teaching.retryPlan':
    'Truy cập mô hình hoặc lập kế hoạch thất bại. Hãy kết nối lại nếu quyền đã hết hạn rồi chủ động lập lại kế hoạch. Tiến trình trước đó vẫn được giữ.',
  'teaching.readiness.unknown': 'Chưa rõ',
  'teaching.readiness.available': 'Có sẵn',
  'teaching.readiness.unavailable': 'Không có sẵn',
  'teaching.readiness.unconfigured': 'Chưa cấu hình',
  'teaching.readiness.ready': 'Sẵn sàng',
  'teaching.planAria': 'Kế hoạch hướng dẫn',
  'teaching.steps': 'Các bước của bạn',
  'teaching.resume': 'Tiếp tục hướng dẫn',
  'teaching.pause': 'Tạm dừng hướng dẫn',
  'teaching.continue': 'Tiếp tục — tôi đã sẵn sàng',
  'teaching.manualTools': 'Công cụ hướng dẫn thủ công',
  'teaching.visualGuidance': 'Hướng dẫn trực quan',
  'teaching.control': 'Điều khiển',
  'teaching.chooseControl': 'Chọn một điều khiển đã quan sát',
  'teaching.gesture': 'Thao tác',
  'teaching.destination': 'Điểm đến',
  'teaching.chooseDestination': 'Chọn điểm đến',
  'teaching.direction': 'Hướng',
  'teaching.caption': 'Chú thích',
  'teaching.guidanceLanguage': 'Ngôn ngữ hướng dẫn',
  'teaching.guidanceLanguageNote':
    'Chỉ điều khiển ngôn ngữ bài học. Ngôn ngữ ứng dụng và nhận dạng giọng nói vẫn tách biệt.',
  'teaching.showGuidance': 'Hiển thị / lặp lại hướng dẫn',
  'teaching.expectedLabel': 'Nhãn điều khiển dự kiến',
  'teaching.expectedValue': 'Giá trị dự kiến',
  'teaching.check': 'Tôi đã thử — kiểm tra',
  'teaching.outcome.confirmed': 'Đã xác nhận',
  'teaching.outcome.mismatch': 'Chưa khớp',
  'teaching.outcome.unknown': 'Chưa rõ',
  'teaching.liveUnavailable': 'Không có cập nhật hướng dẫn trực tiếp.',
  'teaching.permissionUnavailable': 'Không thể kiểm tra quyền.',
  'teaching.permissionHelp':
    'Hãy bật quyền quan sát màn hình và trợ năng rồi thử lại.',
  'teaching.proofUnavailable':
    'Hãy cấu hình tài khoản thử nghiệm riêng tư trên thiết bị này trước.',
  'teaching.unavailable':
    'Không có hướng dẫn. Hãy khởi động môi trường, kiểm tra quyền và quan sát lại.',
  'teaching.overlayAria': 'Hướng dẫn {gesture}: {caption}',
  'teaching.overlayStep': 'Bước {current}/{total} · {gesture}',
  'gesture.point': 'Chỉ',
  'gesture.click': 'Nhấp',
  'gesture.drag': 'Kéo',
  'gesture.type': 'Gõ',
  'gesture.scroll': 'Cuộn',
  'gesture.action.point': 'Nhìn ở đây',
  'gesture.action.click': 'Nhấp',
  'gesture.action.drag': 'Kéo',
  'gesture.action.type': 'Nhập',
  'gesture.action.scroll': 'Cuộn',
  'direction.up': 'Lên',
  'direction.down': 'Xuống',
  'direction.left': 'Trái',
  'direction.right': 'Phải',
  'voice.loading': 'Đang tải hướng dẫn bằng giọng nói…',
  'voice.aria': 'Hướng dẫn bằng giọng nói',
  'voice.eyebrow': 'Yêu cầu Tro chỉ cho bạn',
  'voice.heading': 'Giữ hai phím. Hỏi. Thả ra.',
  'voice.enable': 'Bật giọng nói',
  'voice.permissions': 'Kiểm tra quyền',
  'voice.disable': 'Tắt giọng nói',
  'voice.cancel': 'Hủy chỉ dẫn',
  'voice.textLabel': 'Nhập văn bản thay vì nói',
  'voice.textPlaceholder': 'Ví dụ: Chỉ tôi cách mở bảng cài đặt',
  'voice.run': 'Chỉ tôi cách làm',
  'voice.finalInstruction': 'Yêu cầu của bạn',
  'voice.listeningTranscript': 'Bản chép lời đang nghe',
  'voice.clearTranscript': 'Xóa bản chép lời',
  'voice.selectedWindow': 'Cửa sổ đã chọn:',
  'voice.statusUnavailable': 'Không có trạng thái điều khiển bằng giọng nói.',
  'voice.statusUnverified':
    'Không thể xác minh trạng thái điều khiển bằng giọng nói.',
  'voice.requestFailed':
    'Hướng dẫn bằng giọng nói không thể hoàn tất yêu cầu này.',
  'voice.permissionRecovery':
    'Hãy cho phép truy cập micrô và theo dõi bàn phím rồi kiểm tra quyền lại.',
  'voice.phase.disabled': 'Đã tắt',
  'voice.phase.idle': 'Sẵn sàng',
  'voice.phase.listening': 'Đang nghe',
  'voice.phase.transcribing': 'Đang chép lời',
  'voice.phase.dispatching': 'Đang chuẩn bị',
  'voice.phase.planning': 'Đang lập kế hoạch',
  'voice.phase.guiding': 'Đang hướng dẫn',
  'voice.phase.completed': 'Hoàn tất',
  'voice.phase.cancelled': 'Đã hủy',
  'voice.phase.failed': 'Thất bại',
  'voice.status.fallback': 'Trạng thái hướng dẫn bằng giọng nói đã thay đổi.',
  'workspace.eyebrow': 'Quyền hạn không gian làm việc',
  'workspace.heading': 'Những người có quyền truy cập',
  'workspace.assigned': '{count} người được cấp',
  'workspace.description':
    'Thêm đúng email Google. Tro tự động kết nối sau khi tài khoản đăng nhập—không cần liên kết mời.',
  'workspace.email': 'Email Google',
  'workspace.role': 'Vai trò',
  'workspace.saving': 'Đang lưu…',
  'workspace.add': 'Thêm quyền truy cập',
  'workspace.loading': 'Đang tải thành viên…',
  'workspace.confirmRemoval': 'Xác nhận xóa',
  'workspace.remove': 'Xóa quyền truy cập',
  'workspace.invalidEmail': 'Nhập đúng email Google của thành viên này.',
  'workspace.loadError': 'Không thể tải thành viên. Hãy thử lại.',
  'workspace.added':
    'Đã thêm quyền. Tro sẽ kết nối tài khoản sau khi Google xác minh email đó.',
  'workspace.addError':
    'Không thể thêm quyền. Hãy kiểm tra email và vai trò hiện có.',
  'workspace.confirmMessage': 'Xác nhận xóa quyền của {email}.',
  'workspace.removed': 'Đã xóa quyền truy cập không gian làm việc.',
  'workspace.removeError': 'Không thể xóa quyền truy cập. Hãy thử lại.',
};

const catalogs: Record<AppLocale, Record<TranslationKey, string>> = {
  en: english,
  vi: vietnamese,
};

export function translate(
  locale: AppLocale,
  key: TranslationKey,
  values?: Record<string, string | number>,
) {
  return interpolate(catalogs[locale][key], values);
}

interface LanguageContextValue {
  locale: AppLocale;
  setLocale: (locale: AppLocale) => void;
  t: (key: TranslationKey, values?: Record<string, string | number>) => string;
  localizeMessage: (message: string, fallback: TranslationKey) => string;
}

const defaultContext: LanguageContextValue = {
  locale: 'en',
  setLocale: () => undefined,
  t: (key, values) => translate('en', key, values),
  localizeMessage: (message) => message,
};

const LanguageContext = createContext<LanguageContextValue>(defaultContext);

export function LanguageProvider({
  children,
  initialLocale,
}: {
  children: ReactNode;
  initialLocale?: AppLocale;
}) {
  const [locale, setLocaleState] = useState<AppLocale>(
    () => initialLocale ?? readLocale(),
  );

  useEffect(() => {
    document.documentElement.lang = locale;
  }, [locale]);

  useEffect(() => {
    const receive = (event: StorageEvent) => {
      if (event.key === STORAGE_KEY && isLocale(event.newValue)) {
        setLocaleState(event.newValue);
      }
    };
    window.addEventListener('storage', receive);
    return () => window.removeEventListener('storage', receive);
  }, []);

  const setLocale = useCallback((next: AppLocale) => {
    setLocaleState(next);
    try {
      window.localStorage.setItem(STORAGE_KEY, next);
    } catch {
      // The in-memory preference still applies when storage is unavailable.
    }
  }, []);

  const t = useCallback(
    (key: TranslationKey, values?: Record<string, string | number>) =>
      translate(locale, key, values),
    [locale],
  );

  const localizeMessage = useCallback(
    (message: string, fallback: TranslationKey) => {
      if (locale === 'en') return message;
      const key = knownMessages[message];
      if (key) return catalogs.vi[key];
      const holdMatch = /^Hold (.+) to speak\.$/.exec(message);
      if (holdMatch) return `Giữ ${holdMatch[1]} để nói.`;
      const queueMatch =
        /^Working on the current instruction\. (\d+) follow-ups? queued\.$/.exec(
          message,
        );
      if (queueMatch)
        return `Đang thực hiện chỉ dẫn hiện tại. Có ${queueMatch[1]} chỉ dẫn tiếp theo trong hàng đợi.`;
      return catalogs.vi[fallback];
    },
    [locale],
  );

  const value = useMemo(
    () => ({ locale, setLocale, t, localizeMessage }),
    [locale, localizeMessage, setLocale, t],
  );
  return (
    <LanguageContext.Provider value={value}>
      {children}
    </LanguageContext.Provider>
  );
}

export function useLanguage() {
  return useContext(LanguageContext);
}

export function LanguageSelect({ compact = false }: { compact?: boolean }) {
  const { locale, setLocale, t } = useLanguage();
  return (
    <label
      className={compact ? 'language-select is-compact' : 'language-select'}
    >
      <span>{t('language.label')}</span>
      <select
        aria-label={t('language.label')}
        onChange={(event) => setLocale(event.target.value as AppLocale)}
        value={locale}
      >
        <option value="en">{t('language.english')}</option>
        <option value="vi">{t('language.vietnamese')}</option>
      </select>
    </label>
  );
}

export function localeKey(
  prefix:
    | 'role'
    | 'member'
    | 'runtime.state'
    | 'voice.phase'
    | 'teaching.readiness'
    | 'teaching.outcome'
    | 'onboarding.status',
  value: string,
): TranslationKey | null {
  const key = `${prefix}.${value}` as TranslationKey;
  return key in english ? key : null;
}

function readLocale(): AppLocale {
  try {
    const stored = window.localStorage.getItem(STORAGE_KEY);
    if (isLocale(stored)) return stored;
  } catch {
    // Fall through to the browser language.
  }
  return navigator.language.toLowerCase().startsWith('vi') ? 'vi' : 'en';
}

function isLocale(value: string | null): value is AppLocale {
  return value === 'en' || value === 'vi';
}

function interpolate(
  template: string,
  values?: Record<string, string | number>,
) {
  if (!values) return template;
  return template.replace(/\{(\w+)\}/g, (match, name: string) =>
    Object.hasOwn(values, name) ? String(values[name]) : match,
  );
}

const knownMessages: Partial<Record<string, TranslationKey>> = {
  'Checking this device for a secure session…': 'auth.status.checking',
  'Sign in to continue to your Tro workspace.': 'auth.status.signedOut',
  'Waiting for Google sign-in in your browser…': 'auth.status.signingIn',
  'Signed in securely.': 'auth.status.authenticated',
  'This account is not in an active workspace yet.':
    'auth.status.membershipRequired',
  'Ask a workspace owner to add this exact Google email, then retry.':
    'auth.status.membershipRequired',
  'This account no longer has active workspace access.':
    'auth.status.membershipRequired',
  'Tro cannot reach the sign-in service right now.': 'auth.status.offline',
  'Google sign-in is not configured for this build.': 'auth.notConfigured',
  'Ready to start a diagnostic session.': 'runtime.status.fallback',
  'Ready to preview a diagnostic session.': 'runtime.status.fallback',
  'Starting private runtime…': 'runtime.state.starting',
  'Stopping runtime…': 'runtime.state.stopping',
  'Runtime stopped.': 'runtime.state.stopped',
  'Enable Tro in macOS Privacy & Security → Accessibility and Screen Recording. Relaunch after changes.':
    'teaching.permissionHelp',
  'Selected-window observation checks access when you press Observe. Protected or elevated windows may be unavailable.':
    'teaching.permissionHelp',
  'Enable voice guidance to use push-to-talk.': 'voice.phase.disabled',
  'Voice permissions are required.': 'voice.permissionRecovery',
  'Listening…': 'voice.phase.listening',
  'Finishing your question…': 'voice.phase.transcribing',
  'Finalizing the instruction…': 'voice.phase.transcribing',
  'Preparing the selected window…': 'voice.phase.dispatching',
  'Preparing a simple walkthrough…': 'voice.phase.planning',
  'Follow the cursor in the selected window.': 'voice.phase.guiding',
  'Your guidance is ready.': 'voice.phase.completed',
  'Voice guidance cancelled.': 'voice.phase.cancelled',
  'Voice guidance is disabled.': 'voice.phase.disabled',
  'Voice guidance is disabled until you sign in again.': 'voice.phase.disabled',
};
