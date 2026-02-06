import { createRenderer } from './render/pixiRenderer';
import { MockSimBridge } from './engine/simBridge';

const mount = document.getElementById('app');
if (!mount) throw new Error('Missing #app mount element');

const sim = new MockSimBridge();
await sim.init();
await createRenderer(mount);

window.setInterval(() => {
  sim.step();
  sim.getSnapshot();
}, 50);
