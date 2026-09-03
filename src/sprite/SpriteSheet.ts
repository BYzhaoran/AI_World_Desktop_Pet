export interface SpriteGrid { width: number; height: number; columns: number; rows: number; }
export interface SpriteFrame { index: number; x: number; y: number; width: number; height: number; }

export function frameFor(grid: SpriteGrid, index: number): SpriteFrame {
  if (!Number.isInteger(index) || index < 0 || index >= grid.columns * grid.rows) throw new RangeError('Sprite frame is outside the atlas');
  const column = index % grid.columns;
  const row = Math.floor(index / grid.columns);
  const x = Math.floor(column * grid.width / grid.columns);
  const y = Math.floor(row * grid.height / grid.rows);
  const right = Math.floor((column + 1) * grid.width / grid.columns);
  const bottom = Math.floor((row + 1) * grid.height / grid.rows);
  return { index, x, y, width: right - x, height: bottom - y };
}

export function validateGrid(grid: SpriteGrid): void {
  if (grid.width < grid.columns || grid.height < grid.rows || grid.columns < 1 || grid.rows < 1) throw new Error('Sprite dimensions must contain the configured grid');
}
