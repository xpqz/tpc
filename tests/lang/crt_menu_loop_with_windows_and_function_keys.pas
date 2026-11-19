program MenuLoop;
uses Crt;
var
  Key: Char;
  Counter: Integer;
  Done: Boolean;

procedure Status(Msg: string);
begin
  Window(1, 25, 80, 25);
  TextAttr := $70;
  ClrScr;
  Write(Msg);
  Window(1, 1, 80, 25);
end;

procedure ShowCounter;
begin
  Window(40, 3, 60, 8);
  TextAttr := $1F;
  ClrScr;
  WriteLn('Counter');
  WriteLn;
  Write(Counter:5);
  Window(1, 1, 80, 25);
end;

begin
  ClrScr;
  TextAttr := $07;
  GotoXY(2, 2);
  WriteLn('F1 add  F2 sub  F3 reset  Esc quit');
  Counter := 0;
  Done := False;
  Status('ready');
  ShowCounter;
  repeat
    Key := ReadKey;
    if Key = #0 then
    begin
      Key := ReadKey;
      case Key of
        #59: begin Inc(Counter); Status('added'); end;
        #60: begin Dec(Counter); Status('subtracted'); end;
        #61: begin Counter := 0; Status('reset'); end;
      else
        Status('unknown key');
      end;
    end
    else if Key = #27 then
      Done := True
    else
      Status('plain key ' + Key);
    ShowCounter;
  until Done;
  GotoXY(1, 12);
  TextAttr := $07;
  Write('bye ', Counter);
end.
