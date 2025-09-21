import { ToolboxIdlProgram } from '../src/ToolboxIdlProgram';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount: {
        fields: [
          { name: 'bytes', type: 'bytes' },
          { name: 'vec_u8', type: { vec: 'u8' } },
          { name: 'arr_u8', type: ['u8', 18] },
        ],
      },
    },
  });
  // Check that we can use the manual IDL to encode/decode our account in different ways
  const idlAccount = idlProgram.accounts.get('MyAccount')!;
  const bytesCoordinatorJoinRun = [
    67, 111, 111, 114, 100, 105, 110, 97, 116, 111, 114, 74, 111, 105, 110, 82,
    117, 110,
  ];
  const case1 = idlAccount.encode({
    bytes: bytesCoordinatorJoinRun,
    vec_u8: bytesCoordinatorJoinRun,
    arr_u8: bytesCoordinatorJoinRun,
  });
  const case2 = idlAccount.encode({
    bytes: 'CoordinatorJoinRun',
    vec_u8: 'CoordinatorJoinRun',
    arr_u8: 'CoordinatorJoinRun',
  });
  const case3 = idlAccount.encode({
    bytes: { utf8: 'CoordinatorJoinRun' },
    vec_u8: { utf8: 'CoordinatorJoinRun' },
    arr_u8: { utf8: 'CoordinatorJoinRun' },
  });
  const case4 = idlAccount.encode({
    bytes: { base16: '436F6F7264696E61746F724A6F696E52756E' },
    vec_u8: { base58: '3oEADzTpGyQHQioFsuM8mzvXf' },
    arr_u8: { base64: 'Q29vcmRpbmF0b3JKb2luUnVu' },
  });
  const case5 = idlAccount.encode({
    bytes: {
      value: ['Coordinator', 'Join', 'Run'],
      type: ['string'],
    },
    vec_u8: {
      value: ['Coordinator', 'Join', [82, 117, 110]],
      type: ['bytes', 3],
    },
    arr_u8: {
      value: ['Coordinator', 'Join', 'Run'],
      type: { vec: 'string' },
      prefixed: false,
    },
  });
  // Check that we got the correct results
  const parts = [];
  parts.push(idlAccount.discriminator);
  parts.push(Buffer.from([18, 0, 0, 0]));
  parts.push(Buffer.from('CoordinatorJoinRun', 'utf-8'));
  parts.push(Buffer.from([18, 0, 0, 0]));
  parts.push(Buffer.from('CoordinatorJoinRun', 'utf-8'));
  parts.push(Buffer.from('CoordinatorJoinRun', 'utf-8'));
  const expected = Buffer.concat(parts);
  expect(case1).toStrictEqual(expected);
  expect(case2).toStrictEqual(expected);
  expect(case3).toStrictEqual(expected);
  expect(case4).toStrictEqual(expected);
  expect(case5).toStrictEqual(expected);
});
