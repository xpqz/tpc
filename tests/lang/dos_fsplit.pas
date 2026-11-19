program DosFSplit;
uses Dos;
var
  d: DirStr;
  n: NameStr;
  e: ExtStr;

procedure S(p: PathStr);
begin
  FSplit(p, d, n, e);
  WriteLn('[', d, '][', n, '][', e, ']');
end;

begin
  S('C:\DIR\SUB\FILE.TXT');
  S('file');
  S('.ext');
  S('a.b.c');
  S('\x\');
  S('C:file.');
  S('');
  S('abcdefghijk.abcdef');
  S('c:\a.b\c');
  S('a:b:c.d');
  S('x\y.');
  S('c:\aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa\f.e');
  S('a.b\');
  S('..');
  S('c:.\x');
end.
