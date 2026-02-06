export interface StampedCommand {
  tick: number;
  player_id: { 0: number };
  sequence: number;
  command: unknown;
}

export interface SimBridge {
  init(): Promise<void>;
  enqueueCommands(commands: StampedCommand[]): void;
  step(): void;
  getSnapshot(): unknown;
}

export class MockSimBridge implements SimBridge {
  async init(): Promise<void> {}
  enqueueCommands(_commands: StampedCommand[]): void {}
  step(): void {}
  getSnapshot(): unknown {
    return { tick: 0, entities: [], players: [] };
  }
}
