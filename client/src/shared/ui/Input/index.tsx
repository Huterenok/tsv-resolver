import { FC, InputHTMLAttributes } from "react";

import styles from "./Input.module.css";
import classNames from "classnames";

interface InputProps extends InputHTMLAttributes<HTMLInputElement> {}

export const Input: FC<InputProps> = ({
  onChange,
  value,
  placeholder,
  name,
  type = "text",
  className,
}) => {
  return (
    <input
      type={type}
      onChange={onChange}
      value={value}
      name={name}
      placeholder={placeholder}
      className={classNames(styles.input, className)}
    />
  );
};
