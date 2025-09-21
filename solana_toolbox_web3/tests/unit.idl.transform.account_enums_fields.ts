import { ToolboxIdlProgram } from '../src';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount: {
        discriminator: [77, 78],
        fields: ['MyEnum', 'MyEnum', 'MyEnum', 'MyEnum', 'MyEnum'],
      },
    },
    types: {
      MyEnum: {
        variants128: [
          {
            name: 'Named',
            fields: [{ name: 'field1', type: 'u32' }],
          },
          {
            name: 'Unnamed',
            code: 99,
            fields: ['u8', 'u8'],
          },
          {
            name: 'Empty',
          },
          'Shortened',
          {
            name: 'BigCode',
            code: 0xffffffffffffffffffffffffffffffffn.toString(),
          },
        ],
      },
    },
  });
  // MyAccount info
  const idlAccount = idlProgram.accounts.get('MyAccount')!;
  const accountState = [
    'Empty',
    { Named: { field1: 42 } },
    { Unnamed: [22, 23] },
    'Shortened',
    'BigCode',
  ];
  // Check that we can use the manual IDL to encode/decode our account
  const accountData = idlAccount.encode(accountState);
  expect(accountData).toStrictEqual(
    Buffer.from(
      [
        [77, 78],
        [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [],
        [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [42, 0, 0, 0],
        [99, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [22, 23],
        [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
        [],
        [ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff, ff],
        [],
      ].flat(),
    ),
  );
  expect(idlAccount.decode(accountData)).toStrictEqual(accountState);
});

const ff = 0xff;
