// place files you want to import through the `$lib` alias in this folder.
export type TConfig={
  baseLocation: string,
  folders: TFolder[],
  actions: TAction[],
  projectTemplates:TTemplate[]
};

export type TAction={
  name:string,
  arcuments:string[],
  commands: string[]
};

export type TFolder={
  name:string,
  folder:string
};

export type TTemplate={
  name:string,
  arguments: string[],
  commands: string[]
};
