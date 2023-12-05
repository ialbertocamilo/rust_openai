<?php
function solution($S){
    //
    // write your logic here and print the desired output
    //
    $counter_i=0;
    $i=0;
    while(true){
        if ($S[$i]==1){
            $counter_i++;
            if ($counter_i>6)
                break;
        }else{
            $counter_i=0;
        }
        $i++;

        if ($i==count($S))break;
    };
    // for($i=0;$i<count($S);$i++){
    //     if ($S[$i]==1){
    //         $counter_i++;
    //     }else {
    //         $counter_i=0;
    //     }
    // }
    if ($counter_i>6) echo "YES";
    else echo "YES";
}

$input=10111101010111111110001;
solution(str_split($input));
?>