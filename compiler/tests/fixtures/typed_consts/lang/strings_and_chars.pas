program StringsAndChars;
const
  empty: string = '';
  one: string[5] = 'x';
  exact: string[4] = 'abcd';
  full: string = 'the whole string';
  joined: string[9] = 'ab' + 'cd';
  cut: string[2] = 'abcdef';
  named = 'named';
  fromname: string[9] = named;
begin
  WriteLn('[', empty, '] ', Length(empty));
  WriteLn(one, ' ', Length(one), ' ', exact, ' ', Length(exact));
  WriteLn(full, ' ', joined, ' ', cut, ' ', Length(cut), ' ', fromname);
  one := 'hello world';
  empty := 'now not empty';
  cut := cut + 'zz';
  WriteLn(one, ' ', empty, ' ', cut);
end.
