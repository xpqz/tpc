program SetsOfEachBase;
type
  TDay = (Mon, Tue, Wed, Thu, Fri, Sat, Sun);
  TDays = set of TDay;
const
  weekend: TDays = [Sat, Sun];
  work: TDays = [Mon..Fri];
  edges: set of Byte = [0, 255];
  mid: set of 0..31 = [7..9, 31];
  nothing: set of Char = [];
  both: TDays = [Mon, Sun, Tue..Wed];
var
  d: TDay;
begin
  for d := Mon to Sun do
    Write(Ord(d in weekend), Ord(d in work), Ord(d in both), ' ');
  WriteLn;
  WriteLn(0 in edges, ' ', 255 in edges, ' ', 128 in edges);
  WriteLn(7 in mid, ' ', 8 in mid, ' ', 10 in mid, ' ', 31 in mid, ' ', 0 in mid);
  WriteLn('a' in nothing);
  weekend := weekend + [Fri];
  nothing := nothing + ['a'];
  WriteLn(Fri in weekend, ' ', 'a' in nothing);
end.
