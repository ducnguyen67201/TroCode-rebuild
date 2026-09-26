import { forwardRef } from 'react';

/** Pure UI: a small instructor mark with no event or computer-input capability. */
export const InstructorPointer = forwardRef<HTMLDivElement>(
  function InstructorPointer(_props, ref) {
    return (
      <div ref={ref} className="instructor-pointer">
        <svg viewBox="0 0 32 32">
          <path
            className="instructor-pointer-glyph"
            d="M 9 6 L 24 21 L 18 21 L 16 27 L 12 25 L 14 19 L 7 21 Z"
          />
          <circle className="instructor-pointer-spark" cx="25" cy="7" r="3" />
        </svg>
      </div>
    );
  },
);
