let lines =
  let file = "./bin/input.txt" in
  let ic = In_channel.open_bin file in
  let rec read_lines acc =
    match In_channel.input_line ic with
    | Some line -> read_lines (line :: acc)
    | None -> acc
  in
  let result = read_lines [] in
  In_channel.close ic;
  result

let boxes, instructions =
  let rec parse_line acc lines =
    match lines with
    | hd :: tl ->
        if hd = " 1   2   3   4   5   6   7   8   9 " then (tl, acc)
        else parse_line (hd :: acc) tl
    | [] -> assert false
  in
  parse_line [] lines

(*Go from the array of chars that show the rows to the data structured in cols*)
let box_cols =
  let rec parse_box_lines colsAcc colAcc lines =
    match lines with
    | [] -> colsAcc
    | hd :: tl ->
        let cols =
          match colsAcc with
          | [ h1; h2; h3; h4; h5; h6; h7; h8; h9 ] ->
              [
                (let h = String.get hd 1 in
                 if h = ' ' then h1 else h :: h1);
                (let h = String.get hd 5 in
                 if h = ' ' then h2 else h :: h2);
                (let h = String.get hd 9 in
                 if h = ' ' then h3 else h :: h3);
                (let h = String.get hd 13 in
                 if h = ' ' then h4 else h :: h4);
                (let h = String.get hd 17 in
                 if h = ' ' then h5 else h :: h5);
                (let h = String.get hd 21 in
                 if h = ' ' then h6 else h :: h6);
                (let h = String.get hd 25 in
                 if h = ' ' then h7 else h :: h7);
                (let h = String.get hd 29 in
                 if h = ' ' then h8 else h :: h8);
                (let h = String.get hd 33 in
                 if h = ' ' then h9 else h :: h9);
              ]
          | _ -> assert false
        in
        parse_box_lines cols colAcc tl
  in
  let empty_cols = [ []; []; []; []; []; []; []; []; [] ] in
  parse_box_lines empty_cols [] boxes

let print_top col = print_char (List.nth col 0)

let _print_column col =
  List.iter print_char col;
  print_newline ()

let rec move_n_from_to_p1 num from dest cols =
  if num = 0 then cols
  else
    match cols with
    | [ c1; c2; c3; c4; c5; c6; c7; c8; c9 ] ->
        let box, (c1, c2, c3, c4, c5, c6, c7, c8, c9) =
          match from with
          | 1 -> (List.hd c1, (List.tl c1, c2, c3, c4, c5, c6, c7, c8, c9))
          | 2 -> (List.hd c2, (c1, List.tl c2, c3, c4, c5, c6, c7, c8, c9))
          | 3 -> (List.hd c3, (c1, c2, List.tl c3, c4, c5, c6, c7, c8, c9))
          | 4 -> (List.hd c4, (c1, c2, c3, List.tl c4, c5, c6, c7, c8, c9))
          | 5 -> (List.hd c5, (c1, c2, c3, c4, List.tl c5, c6, c7, c8, c9))
          | 6 -> (List.hd c6, (c1, c2, c3, c4, c5, List.tl c6, c7, c8, c9))
          | 7 -> (List.hd c7, (c1, c2, c3, c4, c5, c6, List.tl c7, c8, c9))
          | 8 -> (List.hd c8, (c1, c2, c3, c4, c5, c6, c7, List.tl c8, c9))
          | 9 -> (List.hd c9, (c1, c2, c3, c4, c5, c6, c7, c8, List.tl c9))
          | _ -> assert false
        in
        let cols =
          match dest with
          | 1 -> [ box :: c1; c2; c3; c4; c5; c6; c7; c8; c9 ]
          | 2 -> [ c1; box :: c2; c3; c4; c5; c6; c7; c8; c9 ]
          | 3 -> [ c1; c2; box :: c3; c4; c5; c6; c7; c8; c9 ]
          | 4 -> [ c1; c2; c3; box :: c4; c5; c6; c7; c8; c9 ]
          | 5 -> [ c1; c2; c3; c4; box :: c5; c6; c7; c8; c9 ]
          | 6 -> [ c1; c2; c3; c4; c5; box :: c6; c7; c8; c9 ]
          | 7 -> [ c1; c2; c3; c4; c5; c6; box :: c7; c8; c9 ]
          | 8 -> [ c1; c2; c3; c4; c5; c6; c7; box :: c8; c9 ]
          | 9 -> [ c1; c2; c3; c4; c5; c6; c7; c8; box :: c9 ]
          | _ -> assert false
        in
        move_n_from_to_p1 (num - 1) from dest cols
    | [ _ ] -> assert false
    | _ -> assert false

let rec parse_inst_p1 cols inst =
  match inst with
  | [] -> cols
  | hd :: tl ->
      if hd = "" then parse_inst_p1 cols tl
      else
        let num, from, dest =
          let str_parts = String.split_on_char ' ' hd in
          ( int_of_string (List.nth str_parts 1),
            int_of_string (List.nth str_parts 3),
            int_of_string (List.nth str_parts 5) )
        in
        let cols = move_n_from_to_p1 num from dest cols in
        parse_inst_p1 cols tl
;;

let cols = parse_inst_p1 box_cols instructions in
List.iter print_top cols

let rec move_n_from_to_p2 acc num from dest cols =
  match cols with
  | [ c1; c2; c3; c4; c5; c6; c7; c8; c9 ] ->
      if num = 0 then
        let acc = List.rev acc in
        match dest with
        | 1 -> [ acc @ c1; c2; c3; c4; c5; c6; c7; c8; c9 ]
        | 2 -> [ c1; acc @ c2; c3; c4; c5; c6; c7; c8; c9 ]
        | 3 -> [ c1; c2; acc @ c3; c4; c5; c6; c7; c8; c9 ]
        | 4 -> [ c1; c2; c3; acc @ c4; c5; c6; c7; c8; c9 ]
        | 5 -> [ c1; c2; c3; c4; acc @ c5; c6; c7; c8; c9 ]
        | 6 -> [ c1; c2; c3; c4; c5; acc @ c6; c7; c8; c9 ]
        | 7 -> [ c1; c2; c3; c4; c5; c6; acc @ c7; c8; c9 ]
        | 8 -> [ c1; c2; c3; c4; c5; c6; c7; acc @ c8; c9 ]
        | 9 -> [ c1; c2; c3; c4; c5; c6; c7; c8; acc @ c9 ]
        | _ -> assert false
      else
        let box, cols =
          match from with
          | 1 -> (List.hd c1, [ List.tl c1; c2; c3; c4; c5; c6; c7; c8; c9 ])
          | 2 -> (List.hd c2, [ c1; List.tl c2; c3; c4; c5; c6; c7; c8; c9 ])
          | 3 -> (List.hd c3, [ c1; c2; List.tl c3; c4; c5; c6; c7; c8; c9 ])
          | 4 -> (List.hd c4, [ c1; c2; c3; List.tl c4; c5; c6; c7; c8; c9 ])
          | 5 -> (List.hd c5, [ c1; c2; c3; c4; List.tl c5; c6; c7; c8; c9 ])
          | 6 -> (List.hd c6, [ c1; c2; c3; c4; c5; List.tl c6; c7; c8; c9 ])
          | 7 -> (List.hd c7, [ c1; c2; c3; c4; c5; c6; List.tl c7; c8; c9 ])
          | 8 -> (List.hd c8, [ c1; c2; c3; c4; c5; c6; c7; List.tl c8; c9 ])
          | 9 -> (List.hd c9, [ c1; c2; c3; c4; c5; c6; c7; c8; List.tl c9 ])
          | _ -> assert false
        in
        let acc = if box == ' ' then acc else box :: acc in
        move_n_from_to_p2 acc (num - 1) from dest cols
  | _ -> assert false

let rec parse_inst_p2 cols inst =
  match inst with
  | [] -> cols
  | hd :: tl ->
      if hd = "" then parse_inst_p2 cols tl
      else
        let num, from, dest =
          let str_parts = String.split_on_char ' ' hd in
          ( int_of_string (List.nth str_parts 1),
            int_of_string (List.nth str_parts 3),
            int_of_string (List.nth str_parts 5) )
        in
        let cols = move_n_from_to_p2 [] num from dest cols in
        parse_inst_p2 cols tl
;;

print_newline ();
print_endline "================";

let cols = parse_inst_p2 box_cols instructions in
List.iter print_top cols;
print_newline ()
