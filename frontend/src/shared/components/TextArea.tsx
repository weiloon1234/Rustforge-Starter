import { forwardRef, useId, type TextareaHTMLAttributes } from "react";

export interface TextAreaProps extends TextareaHTMLAttributes<HTMLTextAreaElement> {
  label?: string;
  error?: string;
  notes?: string;
}

export const TextArea = forwardRef<HTMLTextAreaElement, TextAreaProps>(
  ({ label, error, notes, required, className, id: externalId, ...rest }, ref) => {
    const autoId = useId();
    const id = externalId ?? autoId;

    return (
      <div className="rf-field">
        {label && (
          <label htmlFor={id} className={`rf-label ${required ? "rf-label-required" : ""}`}>
            {label}
          </label>
        )}
        <textarea
          ref={ref}
          id={id}
          required={required}
          className={`rf-textarea ${error ? "rf-textarea-error" : ""} ${className ?? ""}`}
          {...rest}
        />
        {error && <p className="rf-error-message">{error}</p>}
        {notes && !error && <p className="rf-note">{notes}</p>}
      </div>
    );
  },
);

TextArea.displayName = "TextArea";
