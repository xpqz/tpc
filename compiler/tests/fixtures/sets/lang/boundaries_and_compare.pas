program SetBoundaries;
type BS = set of Byte; SS = set of 'a'..'f';
var s, t, e: BS; lo, hi: Integer; q: SS;
begin
  e := [];
  s := [0]; t := [255];
  WriteLn(0 in s, ' ', 255 in s, ' ', 0 in t, ' ', 255 in t, ' ', 1 in s, ' ', 254 in t);
  s := [0, 31, 32, 63, 64, 127, 128, 255];
  WriteLn(31 in s, ' ', 32 in s, ' ', 63 in s, ' ', 64 in s, ' ', 127 in s, ' ', 128 in s, ' ', 129 in s);
  t := [30..33];
  WriteLn(30 in s + t, ' ', 31 in s * t, ' ', 33 in s * t, ' ', 33 in t - s, ' ', 32 in t - s);
  WriteLn(e = [], ' ', e <= s, ' ', s >= e, ' ', s <= e, ' ', e >= s, ' ', e <> s, ' ', [] <> []);
  WriteLn(s = s, ' ', s <= s, ' ', s >= s, ' ', s <> s, ' ', [0..255] >= s, ' ', [0..254] >= s);
  WriteLn([1, 2] <= [1, 2, 3], ' ', [1, 2, 3] <= [1, 2], ' ', [1, 2] >= [1, 2, 3], ' ', [1, 2, 3] >= [1, 2]);
  WriteLn([1, 3] = [3, 1], ' ', [1..3] = [1, 2, 3], ' ', [3..1] = [], ' ', [5..5] = [5]);
  lo := 10; hi := 5;
  s := [lo..hi];
  WriteLn(s = [], ' ', 7 in s);
  s := [hi..lo, 20];
  WriteLn(5 in s, ' ', 10 in s, ' ', 11 in s, ' ', 20 in s, ' ', 4 in s);
  q := ['b'..'d'];
  WriteLn('c' in q, ' ', 'a' in q, ' ', q = ['d', 'b', 'c'], ' ', q <= ['a'..'f'], ' ', ['a'..'f'] <= q);
  q := q + ['f'] - ['b'];
  WriteLn('b' in q, ' ', 'f' in q, ' ', q * ['c'..'f'] = q)
end.
