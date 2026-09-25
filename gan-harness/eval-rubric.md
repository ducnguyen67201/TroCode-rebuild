# Design Evaluation Rubric

Score each category from 0–10, multiply by its weight, and report the weighted
score. Passing score: **7.5/10**. Maximum iterations: **10**.

### Design Quality (weight: 0.35)

- Clear hierarchy and a strong first impression at desktop and 390×844.
- Calm, trustworthy tone suitable for a classroom product.
- Excellent typography, spacing, color, contrast and state legibility.
- Authentication gate feels intentional, not bolted onto the foundation UI.

### Originality (weight: 0.30)

- Memorable visual idea grounded in teaching, wayfinding or focused learning.
- Avoids a generic centered OAuth card, stock gradient blobs or dashboard clichés.
- Uses layout, CSS/SVG detail and motion with restraint and purpose.

### Craft (weight: 0.25)

- Polished responsive behavior, focus states and reduced-motion treatment.
- Accessible semantics, keyboard operation and useful status/error announcements.
- Consistent visual system across signed-out, pending, offline and signed-in states.
- No layout overflow, remote assets, secret exposure or token-like DOM content.

### Functionality (weight: 0.10)

- All required states and actions are represented.
- Protected product controls never render before active membership.
- Busy actions cannot double-submit; subscriptions clean up correctly.
- Existing runtime/teaching UI continues to work after authentication.

## Evaluator output

For each iteration, write:

1. category scores and weighted total;
2. three strongest qualities;
3. up to five concrete deficiencies ordered by impact;
4. a clear PASS/ITERATE verdict;
5. if ITERATE, a concise revision brief for the generator.
