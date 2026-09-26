import {
  cloneElement,
  useId,
  type ButtonHTMLAttributes,
  type ReactElement,
} from 'react';

export function GuidedAction({
  caption,
  active,
  children,
}: {
  caption: string;
  active: boolean;
  children: ReactElement<ButtonHTMLAttributes<HTMLButtonElement>>;
}) {
  const descriptionId = useId();
  const describedBy = [children.props['aria-describedby'], descriptionId]
    .filter(Boolean)
    .join(' ');
  return (
    <div className={active ? 'guided-action is-active' : 'guided-action'}>
      {active && (
        <div className="guided-action-cue">
          <p id={descriptionId}>{caption}</p>
          <svg aria-hidden="true" viewBox="0 0 30 36" focusable="false">
            <path d="M4 2l21 18-10 2-5 11z" />
          </svg>
        </div>
      )}
      {cloneElement(children, {
        'aria-describedby': active ? describedBy : children.props['aria-describedby'],
      })}
    </div>
  );
}
