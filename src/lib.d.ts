declare function screenSizeGet(): Size;

declare function windowPositionGet(windowTitle: string): Position;

declare function windowSizeGet(windowTitle: string): Size;

declare function windowFocus(windowTitle: string): void;

declare function cursorPositionGet(): Point;

declare function cursorPositionSet(point: Point): void;

declare function pixelColorGet(point: Point): string;

declare interface Position {
  top: number;
  left: number;
  bottom: number;
  right: number;
}

declare interface Size {
  width: number;
  height: number;
}

declare interface Point {
  x: number;
  y: number;
}
