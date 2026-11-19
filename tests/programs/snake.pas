{ Snake for the browser terminal. Arrow keys or W A S D steer, Q quits.
  The random numbers come from a fixed-seed generator, so every run places the food at the
  same cells. Screen: row 1 is the score line, the field is columns 1..80, rows 2..25. }
program Snake;

uses Crt;

const
  Left = 1;
  Right = 80;
  Top = 2;
  Bottom = 24;
  MaxLength = 200;
  StepDelay = 80;

var
  SnakeX, SnakeY: array[1..MaxLength] of Integer;
  Length, Score: Integer;
  Dx, Dy: Integer;
  FoodX, FoodY: Integer;
  Seed: LongInt;
  Alive: Boolean;
  Key: Char;

function NextRandom(Limit: Integer): Integer;
begin
  Seed := (Seed * 1103 + 12345) mod 32768;
  NextRandom := Seed mod Limit;
end;

procedure Plot(X, Y: Integer; Ch: Char; Colour: Integer);
begin
  GotoXY(X, Y);
  TextColor(Colour);
  Write(Ch);
end;

procedure ShowScore;
begin
  GotoXY(1, 1);
  TextColor(White);
  Write('Score: ', Score, '   ');
end;

procedure PlaceFood;
var
  I: Integer;
  Free: Boolean;
begin
  repeat
    FoodX := Left + NextRandom(Right - Left + 1);
    FoodY := Top + NextRandom(Bottom - Top + 1);
    Free := True;
    for I := 1 to Length do
      if (SnakeX[I] = FoodX) and (SnakeY[I] = FoodY) then Free := False;
  until Free;
  Plot(FoodX, FoodY, '*', LightRed);
end;

procedure Steer(Wanted: Char);
begin
  case Wanted of
    #72, 'w', 'W': if Dy = 0 then begin Dx := 0; Dy := -1 end;
    #80, 's', 'S': if Dy = 0 then begin Dx := 0; Dy := 1 end;
    #75, 'a', 'A': if Dx = 0 then begin Dx := -1; Dy := 0 end;
    #77, 'd', 'D': if Dx = 0 then begin Dx := 1; Dy := 0 end;
  end;
end;

procedure ReadSteering;
begin
  while KeyPressed do
  begin
    Key := ReadKey;
    if Key = 'q' then Alive := False
    else if Key = #0 then Steer(ReadKey)
    else Steer(Key);
  end;
end;

procedure Step;
var
  NewX, NewY, I: Integer;
  Grew: Boolean;
begin
  NewX := SnakeX[1] + Dx;
  NewY := SnakeY[1] + Dy;
  if (NewX < Left) or (NewX > Right) or (NewY < Top) or (NewY > Bottom) then
  begin
    Alive := False;
    Exit;
  end;
  for I := 1 to Length - 1 do
    if (SnakeX[I] = NewX) and (SnakeY[I] = NewY) then Alive := False;
  if not Alive then Exit;
  Grew := (NewX = FoodX) and (NewY = FoodY);
  if Grew then
  begin
    if Length < MaxLength then Length := Length + 1;
  end
  else
    Plot(SnakeX[Length], SnakeY[Length], ' ', LightGray);
  for I := Length downto 2 do
  begin
    SnakeX[I] := SnakeX[I - 1];
    SnakeY[I] := SnakeY[I - 1];
  end;
  Plot(SnakeX[1], SnakeY[1], 'o', Green);
  SnakeX[1] := NewX;
  SnakeY[1] := NewY;
  Plot(NewX, NewY, 'O', LightGreen);
  if Grew then
  begin
    Score := Score + 1;
    ShowScore;
    PlaceFood;
  end;
end;

begin
  Seed := 4242;
  ClrScr;
  Length := 3;
  Score := 0;
  Dx := 1;
  Dy := 0;
  SnakeX[1] := 10; SnakeY[1] := 10;
  SnakeX[2] := 9;  SnakeY[2] := 10;
  SnakeX[3] := 8;  SnakeY[3] := 10;
  Alive := True;
  ShowScore;
  Plot(10, 10, 'O', LightGreen);
  Plot(9, 10, 'o', Green);
  Plot(8, 10, 'o', Green);
  PlaceFood;
  while Alive do
  begin
    Delay(StepDelay);
    ReadSteering;
    if Alive then Step;
  end;
  GotoXY(1, 25);
  TextColor(White);
  Write('Game over. Score: ', Score);
end.
