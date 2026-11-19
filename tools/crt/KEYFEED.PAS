{ Scripted keyboard. Crt reads keys through BIOS INT 16h (not DOS stdin), so this unit
  replaces INT 16h with a procedure that serves the keys listed in KEYS.DAT: one word per
  line, `$AAAA` hex, high byte the scan code and low byte the ASCII code, as INT 16h returns
  them (tools/crt/keys.awk makes the file from NAME.keys). A blocking read after the last key
  halts the program with exit code 255, which shows in the screen dump. Without KEYS.DAT
  there are no keys. }
unit KeyFeed;
interface
implementation
uses Dos;
const
  KeysExhausted = 255;
var
  Keys: array[1..500] of Word;
  Count, Next: Integer;

procedure LoadKeys;
var
  T: Text;
begin
  Count := 0;
  Next := 1;
  Assign(T, 'KEYS.DAT');
  {$I-}
  Reset(T);
  {$I+}
  if IOResult <> 0 then Exit;
  while not Eof(T) do
  begin
    Inc(Count);
    ReadLn(T, Keys[Count]);
  end;
  Close(T);
end;

{$F+}
procedure Hook(Flags, CS, IP, AX, BX, CX, DX, SI, DI, DS, ES, BP: Word); interrupt;
begin
  case Hi(AX) of
    $00, $10:
      begin
        if Next > Count then Halt(KeysExhausted);
        AX := Keys[Next];
        Inc(Next);
      end;
    $01, $11:
      if Next <= Count then
      begin
        AX := Keys[Next];
        Flags := Flags and not FZero;
      end
      else Flags := Flags or FZero;
  end;
end;
{$F-}

begin
  LoadKeys;
  SetIntVec($16, @Hook);
end.
