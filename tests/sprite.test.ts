import { describe, expect, it } from 'vitest';
import { frameFor, validateGrid } from '../src/sprite/SpriteSheet';
import { AnimationController } from '../src/sprite/AnimationController';
describe('sprite system',()=>{
 it('indexes a configurable atlas',()=>{const grid={width:1536,height:1872,columns:8,rows:9};expect(frameFor(grid,9)).toMatchObject({x:192,y:208,width:192,height:208});});
 it('accepts non-divisible atlas dimensions',()=>{expect(()=>validateGrid({width:1537,height:1873,columns:8,rows:9})).not.toThrow();});
 it('rejects an atlas smaller than its grid',()=>{expect(()=>validateGrid({width:7,height:8,columns:8,rows:9})).toThrow();});
 it('advances and loops animations with monotonic timestamps',()=>{const c=new AnimationController({idle:{frames:[4,5],minFps:2,maxFps:2,loop:true}});expect(c.tick('idle',0)).toBe(4);expect(c.tick('idle',500)).toBe(5);expect(c.tick('idle',1000)).toBe(4);});
});
