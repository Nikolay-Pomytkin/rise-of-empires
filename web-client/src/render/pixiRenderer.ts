import { Application, Text } from 'pixi.js';

export async function createRenderer(container: HTMLElement): Promise<Application> {
  const app = new Application();
  await app.init({ background: '#182028', resizeTo: window });
  container.appendChild(app.canvas);

  const label = new Text({ text: 'Rise RTS Web Client Skeleton', style: { fill: '#ffffff' } });
  label.x = 20;
  label.y = 20;
  app.stage.addChild(label);

  return app;
}
