{ Prototype: scripted keyboard. Replaces BIOS INT 16h so ReadKey, KeyPressed and ReadLn under
  Crt see the characters of Keys instead of the keyboard (Crt reads keys through INT 16h, not
  through DOS stdin). Must come before Crt in the uses clause only for readability; the hook
  is looked up at call time. When Keys runs out, a blocking read reports Enter forever; the
  real unit should halt with a marker instead. Extended keys would need a scan code in Hi(AX). }
unit KeyFeed;
interface
implementation
uses Dos;
const
  Keys: string = 'hello'#13;
var
  Next: Integer;

{$F+}
procedure Hook(Flags, CS, IP, AX, BX, CX, DX, SI, DI, DS, ES, BP: Word); interrupt;
begin
  case Hi(AX) of
    $00, $10:
      begin
        if Next <= Length(Keys) then AX := Ord(Keys[Next]) else AX := $1C0D;
        Inc(Next);
      end;
    $01, $11:
      if Next <= Length(Keys) then
      begin
        AX := Ord(Keys[Next]);
        Flags := Flags and not FZero;
      end
      else Flags := Flags or FZero;
  end;
end;
{$F-}

begin
  Next := 1;
  SetIntVec($16, @Hook);
end.
