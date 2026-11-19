program TypedconstSets;
type
  TColor = (Red, Green, Blue, Black);
  TColors = set of TColor;
const
  digits: set of '0'..'9' = ['0'..'9'];
  vowels: set of Char = ['a', 'e', 'i', 'o', 'u'];
  warm: TColors = [Red];
  none: TColors = [];
  small: set of Byte = [1, 3, 5, 200];
begin
  WriteLn('5' in digits, ' ', 'x' in digits);
  WriteLn('e' in vowels, ' ', 'z' in vowels);
  WriteLn(Red in warm, ' ', Blue in warm, ' ', Red in none);
  WriteLn(1 in small, ' ', 2 in small, ' ', 200 in small);
  warm := warm + [Blue];
  vowels := vowels - ['e'];
  none := [Black];
  small := small + [2] - [1];
  WriteLn(Blue in warm, ' ', 'e' in vowels, ' ', Black in none);
  WriteLn(1 in small, ' ', 2 in small);
end.
