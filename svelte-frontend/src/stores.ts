import { writable } from 'svelte/store'
import type { Writable } from 'svelte/store'
import type { SpriteFrameData } from './spriteframedata';

export const spriteframes:Writable<SpriteFrameData[]> = writable([]);