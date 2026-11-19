program Chords;
{ ReadKey for the keys of the TP IDE's bindings: Ctrl+letter is the control code 1..26,
  Alt, Shift, Ctrl combinations with F-keys, Shift-Tab and Ctrl-cursor keys are 0 plus the
  extended code of the Reference Guide's Table C.2. Each key prints as c<code> for a plain
  code or x<code> for 0 plus an extended code. }
uses Crt;
var
  C: Char;
  Done: Boolean;
begin
  Done := False;
  repeat
    C := ReadKey;
    if C = #0 then
      Write('x', Ord(ReadKey), ' ')
    else
    begin
      Write('c', Ord(C), ' ');
      Done := C = #27;
    end;
  until Done;
end.
