import { render } from '@testing-library/react';
import { expect, test } from 'vitest';
import { InstructorPointer } from './InstructorPointer';

test('renders a compact visual pointer without actionable controls', () => {
  const { container } = render(<InstructorPointer />);

  expect(container.querySelector('.instructor-pointer-glyph')).not.toBeNull();
  expect(container.querySelector('.instructor-pointer-halo')).toBeNull();
  expect(
    container.querySelectorAll('button, input, textarea, select, a'),
  ).toHaveLength(0);
});
