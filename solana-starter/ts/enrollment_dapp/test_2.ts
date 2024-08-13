export interface WbaPrereq {
    address:      string;
    metadata:     Metadata;
    instructions: Instruction[];
    accounts:     WelcomeAccount[];
    errors:       Error[];
    types:        TypeElement[];
}

export interface WelcomeAccount {
    name:          string;
    discriminator: number[];
}

export interface Error {
    code: number;
    name: string;
    msg:  string;
}

export interface Instruction {
    name:          string;
    discriminator: number[];
    accounts:      InstructionAccount[];
    args:          Arg[];
}

export interface InstructionAccount {
    name:      string;
    writable?: boolean;
    signer?:   boolean;
    pda?:      PDA;
    address?:  string;
}

export interface PDA {
    seeds: Seed[];
}

export interface Seed {
    kind:   string;
    value?: number[];
    path?:  string;
}

export interface Arg {
    name: string;
    type: string;
}

export interface Metadata {
    name:        string;
    version:     string;
    spec:        string;
    description: string;
}

export interface TypeElement {
    name: string;
    type: TypeType;
}

export interface TypeType {
    kind:   string;
    fields: Arg[];
}



  export const IDL: WbaPrereq = {
    address: "WBAQSygkwMox2VuWKU133NxFrpDZUBdvSBeaBEue2Jq",
    metadata: {
      name: "wba_prereq",
      version: "0.1.0",
      spec: "0.1.0",
      description: "Created with Anchor",
    },
    instructions: [
      {
        name: "complete",
        discriminator: [0, 77, 224, 147, 136, 25, 88, 76],
        accounts: [
          {
            name: "signer",
            writable: true,
            signer: true,
          },
          {
            name: "prereq",
            writable: true,
            pda: {
              seeds: [
                {
                  kind: "const",
                  value: [112, 114, 101, 114, 101, 113],
                },
                {
                  kind: "account",
                  path: "signer",
                },
              ],
            },
          },
          {
            name: "system_program",
            address: "11111111111111111111111111111111",
            writable: false,
          },
        ],
        args: [
          {
            name: "github",
            type: "bytes",
          },
        ],
      },
      {
        name: "update",
        discriminator: [219, 200, 88, 176, 158, 63, 253, 127],
        accounts: [
          {
            name: "signer",
            writable: true,
            signer: true,
          },
          {
            name: "prereq",
            writable: true,
          },
          {
            name: "system_program",
            address: "11111111111111111111111111111111",
            writable: false,
          },
        ],
        args: [
          {
            name: "github",
            type: "bytes",
          },
        ],
      },
    ],
    accounts: [
      {
        name: "Q2Prereq2024",
        discriminator: [210, 203, 168, 103, 251, 233, 204, 6],
      },
    ],
    errors: [
      {
        code: 6000,
        name: "InvalidGithubAccount",
        msg: "Invalid Github account",
      },
    ],
    types: [
      {
        name: "Q2Prereq2024",
        type: {
          kind: "struct",
          fields: [
            {
              name: "github",
              type: "bytes",
            },
            {
              name: "key",
              type: "pubkey",
            },
          ],
        },
      },
    ],
  };
  