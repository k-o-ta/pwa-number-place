if('serviceWorker' in navigator) {
    navigator.serviceWorker.register('/sw.js');
};
import {Sudoku} from "pwa-number-place";
const sudoku = Sudoku.new();


// var button = document.getElementById("notifications");
// button.addEventListener('click', function(e) {
//     Notification.requestPermission().then(function(result) {
//         if(result === 'granted') {
//             randomNotification();
//         }
//     });
// });
//

var supportTouch = 'ontouchend' in document;
var EVENTNAME_TOUCHEND = supportTouch ? 'touchend' : 'mouseup';

var lock = undefined;


function checkPlace(event) {
  let checked = parseInt(event.target.getAttribute('data-place-index'));
  if (checked === lock) {
    lock = undefined;
    event.target.classList.remove("checked");
  } else {
    if (typeof lock !== 'undefined') {
      Array.from(document.querySelector('#place').children)[lock].classList.remove("checked")
    }
    lock = checked;
    event.target.classList.add("checked");

  }
  console.log(event.target)
  console.log(`check index ${checked}`)
}

function checkNumber(event) {
  if (typeof lock !== 'undefined') {
    if (sudoku.is_fixed(lock)) {
      console.log('fixed');
      return;
    } else {
    let num = parseInt(event.target.getAttribute('data-number'),10);
    let place = Array.from(document.querySelector('#place').children)[lock];
    place.innerHTML = num;
    place.classList.remove("checked");
    sudoku.set_num(lock, num);
    console.log(`set num ${num} to ${lock} wasm ${sudoku.get_num(lock)}`);
    lock = undefined;
    return;
    }
  }
}

Array.from(document.querySelector('#place').children).forEach((div, index) => {
console.log('hoge1');
  div.addEventListener(EVENTNAME_TOUCHEND, checkPlace);
  div.setAttribute('data-place-index', index);
  const num = sudoku.get_num(index);
  if (typeof num !== 'undefined') {
    div.innerHTML = sudoku.get_num(index);
    div.classList.add("fixed");
  }
});

Array.from(document.querySelector('#numbers').children).forEach((div, index) => {
  div.addEventListener(EVENTNAME_TOUCHEND, checkNumber);
});
