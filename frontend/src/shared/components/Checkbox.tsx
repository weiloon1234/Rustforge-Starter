import { forwardRef, useId, type InputHTMLAttributes } from "react";

export interface CheckboxProps extends Omit<InputHTMLAttributes<HTMLInputElement>, "type"> {
  label?: string;
  error?: string;
  notes?: string;
}

export const Checkbox = forwardRef<HTMLInputElement, CheckboxProps>(
  ({ label, error, notes, className, id: externalId, ...rest }, ref) => {
    const autoId = useId();
    const id = externalId ?? autoId;

    return (
      <div className="rf-field">
        <div className="rf-checkbox-wrapper">
          <input
            ref={ref}
            id={id}
            type="checkbox"
            className={`rf-checkbox ${error ? "rf-checkbox-error" : ""} ${className ?? ""}`}
            {...rest}
          />
          {label && (
            <label htmlFor={id} className="rf-checkbox-label">
              {label}
            </label>
          )}
        </div>
        {error && <p className="rf-error-message">{error}</p>}
        {notes && !error && <p className="rf-note">{notes}</p>}
      </div>
    );
  },
);

Checkbox.displayName = "Checkbox";
