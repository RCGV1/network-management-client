import React from "react";
import type { DetailedHTMLProps, InputHTMLAttributes } from "react";

// eslint-disable-next-line @typescript-eslint/no-empty-interface
export interface IConnectionInputProps
  extends DetailedHTMLProps<
    InputHTMLAttributes<HTMLInputElement>,
    HTMLInputElement
  > {}

const ConnectionInput = (props: IConnectionInputProps) => {
  return (
    <input
      className="flex-1 border border-gray-400 rounded-lg px-5 py-4 text-gray-700 placeholder:text-gray-400 h-full bg-transparent focus:outline-none disabled:cursor-wait"
      {...props}
    />
  );
};

export default ConnectionInput;