# Brief: Tro Google sign-in gate

Design and implement the desktop authentication experience for Tro, a calm,
screen-aware teaching assistant. The product must remain completely hidden until
the native host reports an authenticated Google account with active workspace
membership.

Required states:

- checking the secure device session;
- signed out with one clear “Continue with Google” action;
- browser sign-in in progress;
- authenticated but waiting for an owner to add the exact verified email;
- temporarily offline with retry;
- configuration/error state with a safe public message;
- authenticated product shell with account/workspace context and sign out.

Design goals:

- feel warm, focused and quietly premium for teachers and students;
- extend Tro’s existing cream/forest palette without becoming a generic SaaS
  login card;
- make the teaching metaphor visible through subtle wayfinding, pacing and
  layered editorial composition;
- use distinctive typography, spatial rhythm, illustration made from CSS/SVG,
  and restrained motion with a complete reduced-motion path;
- remain readable at 390×844 and desktop widths, keyboard accessible, and clear
  at high zoom;
- never display, decode or persist JWTs/OAuth credentials in React;
- never load remote images, fonts, scripts or Google content in the WebView;
- keep the overlay route presentation-only and outside the auth gate.

Implementation context:

- React 19 + TypeScript + Vite;
- `AuthGate` owns state projection only and calls a token-free `AuthClient`;
- preview mode must simulate every auth state for tests and design review;
- existing runtime and teaching components remain functional after sign-in;
- frontend payloads are validated against the shared auth JSON schema.

Primary success test: before authentication, no runtime or teaching control is
present in the DOM. After simulated authentication, the existing product becomes
available without a reload.
