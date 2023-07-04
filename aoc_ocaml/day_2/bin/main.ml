let lines =
  let file = "./bin/input.txt" in
  let ic = In_channel.open_bin file in
  (* input_all seems to have a max size, so some characters were being left off
     with that, instead we read every single line like this...*)
  let rec read_lines acc =
    match In_channel.input_line ic with
    | Some line -> read_lines (line :: acc)
    | None -> List.rev acc
  in
  let result = read_lines [] in
  In_channel.close ic;
  result
;;

(*
    Choices:
     - X/A Rock | Y/B Paper | Z/C Scissors
     - 1        | 2         | 3
     Outcomes:
     - W: 6   | D: 3    | L: 0
*)

let parse_line_p1 str =
  if String.length str != 0 then
    let a, b = (str.[0], str.[2]) in
    match (a, b) with
    | 'A', b -> ( match b with 'X' -> 4 | 'Y' -> 8 | 'Z' -> 3 | _ -> exit 0)
    | 'B', b -> ( match b with 'X' -> 1 | 'Y' -> 5 | 'Z' -> 9 | _ -> exit 0)
    | 'C', b -> ( match b with 'X' -> 7 | 'Y' -> 2 | 'Z' -> 6 | _ -> exit 0)
    | _, _ -> exit 0
  else exit 0
in

let rec total_points acc ls =
  match ls with
  | [] -> acc
  | hd :: tl -> total_points (parse_line_p1 hd + acc) tl
in

print_string "Total_p1: ";
print_endline (string_of_int (total_points 0 lines));

let parse_line_p2 str =
  if String.length str != 0 then
    let a, b = (str.[0], str.[2]) in
    (*X: lose Y: draw Z: win*)
    match (a, b) with
    | 'A', b -> ( match b with 'X' -> 3 | 'Y' -> 4 | 'Z' -> 8 | _ -> exit 0)
    | 'B', b -> ( match b with 'X' -> 1 | 'Y' -> 5 | 'Z' -> 9 | _ -> exit 0)
    | 'C', b -> ( match b with 'X' -> 2 | 'Y' -> 6 | 'Z' -> 7 | _ -> exit 0)
    | _, _ -> exit 0
  else exit 0
in

let rec total_points_p2 acc ls =
  match ls with
  | [] -> acc
  | hd :: tl -> total_points_p2 (parse_line_p2 hd + acc) tl
in

print_string "Total_p2: ";
print_endline (string_of_int (total_points_p2 0 lines))
