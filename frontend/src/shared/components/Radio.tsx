import { useId } from "react";

export interface RadioOption {
  value: string;
  label: string;
  disabled?: boolean;
}

export interface RadioProps {
  name: string;
  options: RadioOption[];
  value?: string;
  onChange?: (value: string) => void;
  label?: string;
  error?: string;
  notes?: string;
  required?: boolean;
  disabled?: boolean;
  className?: string;
}

export function Radio({ name, options, value, onChange, label, error, notes, required, disabled, className }: RadioProps) {
  const groupId = useId();

  return (
    <div className="rf-field">
      {label && (
        <span className={`rf-label ${required ? "rf-label-required" : ""}`}>
          {label}
        </span>
      )}
      <div role="radiogroup" aria-labelledby={label ? `${groupId}-label` : undefined} className={`rf-radio-group ${className ?? ""}`}>
        {options.map((opt) => {
          const optId = `${groupId}-${opt.value}`;
          return (
            <div key={opt.value} className="rf-radio-wrapper">
              <input
                id={optId}
                type="radio"
                name={name}
                value={opt.value}
                checked={value === opt.value}
                onChange={() => onChange?.(opt.value)}
                disabled={disabled || opt.disabled}
                className={`rf-radio ${error ? "rf-radio-error" : ""}`}
              />
              <label htmlFor={optId} className="rf-radio-label">
                {opt.label}
              </label>
            </div>
          );
        })}
      </div>
      {error && <p className="rf-error-message">{error}</p>}
      {notes && !error && <p className="rf-note">{notes}</p>}
    </div>
  );
}
