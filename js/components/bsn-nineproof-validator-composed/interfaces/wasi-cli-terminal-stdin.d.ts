/** @module Interface wasi:cli/terminal-stdin@0.2.3 **/
export function getTerminalStdin(): TerminalInput | undefined;
export type TerminalInput = import('./wasi-cli-terminal-input.js').TerminalInput;
