# Advent of Code 2023

## Quick setup
``` shell
rustup default stable
cargo install cargo-generate
brew install just
```

## Some commands
Create package for a new day:
```shell
just create <day>
```

Execute binary for a given day and task:
```shell
just run <day> <part>
```
where `part` is either `part01` or `part02`

## Calendar
<pre><span aria-hidden="true">                                                 </span>
<span aria-hidden="true">                                                 </span>
<span aria-hidden="true">           <span>...</span>                                   </span>
<span aria-hidden="true">    <span>.''....'</span> <span>'..</span>        <i>*</i>                          <span>13</span>    | <i>Coming soon ...</i></span>
<span aria-hidden="true">    <span>'.ZZ</span>                                         </span>
<span title="Day 12, two stars" ><span>.''''</span> <span>ZZ</span><b>*</b> <span>.'''.</span>     <span>'</span>                              <span>12</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/12" target="_blank">puzzle</a> | <a href="day-12/src/lib.rs">solution</a></span>
<span aria-hidden="true"><span>'....</span>     <span>'...'</span>     <span>....'</span>                        </span>
<span title="Day 11, two stars" >    <span>.'</span><b>*</b>            <span>'.</span>                              <span>11</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/11" target="_blank">puzzle</a> | <a href="day-11/src/lib.rs">solution</a></span>
<span title="Day 10, two stars" >    <span>'..''''.</span><b>*</b><span>.''''..'</span><span>'</span> <span>''...</span>                       <span>10</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/10" target="_blank">puzzle</a> | <a href="day-10/src/lib.rs">solution</a></span>
<span aria-hidden="true">          <span>.</span><span>'''</span><span>~</span> <span>~</span> <span>~</span> <span>~</span>   <span>###</span> <span>''.</span>                  </span>
<span title="Day 9, two stars" >        <span>.'</span> <span>~</span>  <span>,</span><b>*</b> <span>~</span> <span>~</span> <span>~</span> <span>~</span> <span>#####</span> <span>'.</span>                  <span> 9</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/9" target="_blank">puzzle</a> | <a href="day-09/src/lib.rs">solution</a></span>
<span title="Day 8, two stars" >        <span>:</span> <span>~</span> <span>'</span><span>(~)</span><span>,</span> <span>~</span> <b>*</b> <span>~</span> <span>~</span> <span>~</span> <span>###</span> <span>:</span>                  <span> 8</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/8" target="_blank">puzzle</a> | <a href="day-08/src/lib.rs">solution</a></span>
<span aria-hidden="true">        <span>'.</span> <span>~</span> <span>"</span> <span>'</span> <span>~</span> <span>~</span> <span>~</span>   <span>#####</span> <span>.'</span>                </span>
<span title="Day 7, two stars" >          <span>'..</span> <span>~</span> <span>~</span> <b>*</b> <span>~</span> <span>#####</span> <span>..'</span><span>.'''''''''...</span>       <span> 7</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/7" target="_blank">puzzle</a> | <a href="day-07/src/lib.rs">solution</a></span>
<span title="Day 6, two stars" >             <span>'''.........'''</span><span>'</span> <span>~</span> <span>.'</span><b>*</b><span>.</span> <span>~</span>  <span>..</span>  <span>''.</span>    <span> 6</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/6" target="_blank">puzzle</a> | <a href="day-06/src/lib.rs">solution</a></span>
<span aria-hidden="true">                        <span>.'</span> <span>~</span>    <span>'...'</span> <span>~</span><span>'</span>  <span>'.</span><span>~</span>  <span>'.</span></span>
<span title="Day 5, two stars" >                        <span>:</span>         <span>~</span>     <span>'.</span> <b>*</b><span>'.</span><span>~</span> <span>:</span>  <span> 5</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/5" target="_blank">puzzle</a> | <a href="day-05/src/lib.rs">solution</a></span>
<span aria-hidden="true">                 <span>...''''</span><span>'.</span>         <span>.''.</span><span>~</span>  <span>'..'</span> <span>.'</span></span>
<span title="Day 4, two stars" >              <span>.''</span>         <span>'..</span>  <span>~</span><span>..'</span><b>*</b>   <span>'.</span> <span>~</span> <span>..'</span>    <span> 4</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/4" target="_blank">puzzle</a> | <a href="day-04/src/lib.rs">solution</a></span>
<span aria-hidden="true">            <span>.'</span>               <span>'''..</span><span>/</span><span>......'''</span>     </span>
<span aria-hidden="true">            <span>:</span>             <span>/\</span>    <span>-</span><span>/</span>  <span>:</span>            </span>
<span aria-hidden="true">            <span>'.</span>            <span>-</span>   <span>-</span> <span>/</span>  <span>.'</span>            </span>
<span title="Day 3, two stars" >              <span>'..</span>    <span>-</span>     <span>-</span>   <b>*</b><span>..'</span>                <span> 3</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/3" target="_blank">puzzle</a> | <a href="day-03/src/lib.rs">solution</a></span>
<span title="Day 2, two stars" >    <span>----@</span>        <span>'''..</span><b>*</b><span>......'''</span>                   <span> 2</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/2" target="_blank">puzzle</a> | <a href="day-02/src/lib.rs">solution</a></span>
<span title="Day 1, two stars" >  <b>*</b> <span>!</span> <span>/^\</span>                                          <span> 1</span> <b>*</b><b>*</b> | <a href="https://adventofcode.com/2023/day/1" target="_blank">puzzle</a> | <a href="day-01/src/lib.rs">solution</a></span>
</pre>