import { ToolboxIdlProgram } from '../src';

it('run', () => {
  // Create an IDL on the fly
  const idlProgram = ToolboxIdlProgram.tryParse({
    accounts: {
      MyAccount: {
        fields: [{ name: 'value', type: 'string' }],
      },
    },
  });
  // Various string cases to test
  const longString = 'Long string '.repeat(1000);
  const tests = [
    'Hello world',
    '',
    'Special chars !@#$%^&*()_+-=[]{}|;\':",.<>/?`~',
    'Unicode: 測試, тест, اختبار, परीक्षण',
    longString,
    'Emoji: 😀🚀🌟🔥💧🍕🎉',
    'Multiline:\nLine 1\nLine 2\nLine 3',
    'Whitespace:    \t   \n  ',
    'Control chars: \x00\x01\x02\x03\x04\x05\x06\x07\x08\x09',
    'Mixed: Hello, 世界! 12345 🚀🔥\nNew line and \t tab.',
  ];
  // Check that we can properly serialize various strings
  const idlAccount = idlProgram.accounts.get('MyAccount')!;
  for (const test of tests) {
    const value = { value: test };
    const encoded = idlAccount.encode(value);
    const decoded = idlAccount.decode(encoded);
    expect(value).toStrictEqual(decoded);
  }
});
