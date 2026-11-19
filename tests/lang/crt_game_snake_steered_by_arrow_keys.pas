program SnakeGame;
uses Crt;
const
  Left = 2;
  Top = 2;
  Right = 30;
  Bottom = 12;
  MaxLength = 50;
type
  Cell = record
    X, Y: Integer;
  end;
var
  Body: array[1..MaxLength] of Cell;
  Len, I: Integer;
  Dx, Dy: Integer;
  FoodX, FoodY: Integer;
  Score, Frames: Integer;
  Alive: Boolean;
  Key: Char;
  Head: Cell;

procedure DrawFrame;
var
  X, Y: Integer;
begin
  TextColor(LightGray);
  for X := Left - 1 to Right + 1 do
  begin
    GotoXY(X, Top - 1);
    Write('#');
    GotoXY(X, Bottom + 1);
    Write('#');
  end;
  for Y := Top to Bottom do
  begin
    GotoXY(Left - 1, Y);
    Write('#');
    GotoXY(Right + 1, Y);
    Write('#');
  end;
end;

procedure PlaceFood;
begin
  FoodX := Left + (Score * 7 + 5) mod (Right - Left);
  FoodY := Top + (Score * 3 + 2) mod (Bottom - Top);
  TextColor(LightRed);
  GotoXY(FoodX, FoodY);
  Write('*');
end;

procedure Steer;
begin
  Key := ReadKey;
  if Key = #0 then
  begin
    Key := ReadKey;
    case Key of
      #72: begin Dx := 0; Dy := -1; end;
      #80: begin Dx := 0; Dy := 1; end;
      #75: begin Dx := -1; Dy := 0; end;
      #77: begin Dx := 1; Dy := 0; end;
    end;
  end;
end;

procedure Step;
begin
  Head := Body[1];
  Head.X := Head.X + Dx;
  Head.Y := Head.Y + Dy;
  if (Head.X < Left) or (Head.X > Right) or (Head.Y < Top) or (Head.Y > Bottom) then
  begin
    Alive := False;
    Exit;
  end;
  GotoXY(Body[Len].X, Body[Len].Y);
  Write(' ');
  if (Head.X = FoodX) and (Head.Y = FoodY) then
  begin
    if Len < MaxLength then
      Inc(Len);
    Inc(Score);
    PlaceFood;
  end;
  for I := Len downto 2 do
    Body[I] := Body[I - 1];
  Body[1] := Head;
  TextColor(LightGreen);
  GotoXY(Head.X, Head.Y);
  Write('O');
end;

begin
  ClrScr;
  DrawFrame;
  Len := 3;
  for I := 1 to Len do
  begin
    Body[I].X := 10 - I;
    Body[I].Y := 5;
  end;
  Dx := 1;
  Dy := 0;
  Score := 0;
  Frames := 0;
  Alive := True;
  PlaceFood;
  while Alive do
  begin
    if KeyPressed then
      Steer;
    Step;
    Inc(Frames);
    Delay(100);
  end;
  TextColor(Yellow);
  GotoXY(1, 15);
  WriteLn('Game over: score ', Score, ' length ', Len, ' frames ', Frames);
end.
