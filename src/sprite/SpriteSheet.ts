export interface SpriteGrid { width: number; height: number; columns: number; rows: number; }
export interface SpriteFrame { index: number; x: number; y: number; width: number; height: number; }

export function frameFor(grid: SpriteGrid, index: number): SpriteFrame {
  if (!Number.isInteger(index) || index < 0 || index >= grid.columns * grid.rows) throw new RangeError('Sprite frame is outside the atlas');
  const width = grid.width / grid.columns;
  const height = grid.height / grid.rows;
  return { index, x: (index % grid.columns) * width, y: Math.floor(index / grid.columns) * height, width, height };
}

export function validateGrid(grid: SpriteGrid): void {
  if (grid.columns < 1 || grid.rows < 1 || grid.width % grid.columns !== 0 || grid.height % grid.rows !== 0) throw new Error('Sprite dimensions must divide evenly into the configured grid');
}
