import { describe, expect, it } from 'vitest';
import { frameFor, validateGrid } from '../src/sprite/SpriteSheet';
import { AnimationController } from '../src/sprite/AnimationController';
describe('sprite system',()=>{
 it('indexes a configurable atlas',()=>{const grid={width:1536,height:1872,columns:8,rows:9};expect(frameFor(grid,9)).toMatchObject({x:192,y:208,width:192,height:208});});
 it('rejects invalid dimensions',()=>{expect(()=>validateGrid({width:10,height:10,columns:3,rows:2})).toThrow();});
 it('advances and loops animations',()=>{const c=new AnimationController({idle:{frames:[4,5],fps:2,loop:true}});expect(c.tick('idle',500)).toBe(5);expect(c.tick('idle',500)).toBe(4);});
});
