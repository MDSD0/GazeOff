const palettes=[
  {at:0,c:['#0b162c','#15294a','#213f6a','#3e638e','#7093b5','#b6cbd9'],stars:0},
  {at:.24,c:['#091b42','#0f2e6e','#1b4c9e','#2e76cc','#4a9df2','#82c9ff'],stars:0},
  {at:.52,c:['#141638','#242654','#423d6e','#73566a','#a87e67','#deb081'],stars:0},
  {at:.76,c:['#1a1730','#2d2042','#5e3454','#b56953','#e8a874','#f2d7b8'],stars:.12},
  {at:1,c:['#050714','#0a0c1f','#14122a','#1f1830','#2a1f3a','#3a2a44'],stars:.55}
];
const names=['--zenith','--high','--mid','--low','--horizon','--glow'];
const hex=h=>[parseInt(h.slice(1,3),16),parseInt(h.slice(3,5),16),parseInt(h.slice(5,7),16)];
const mix=(a,b,t)=>{const x=hex(a),y=hex(b);return `rgb(${x.map((v,i)=>Math.round(v+(y[i]-v)*t)).join(',')})`};
function updateSky(){const max=document.documentElement.scrollHeight-innerHeight;const p=max?scrollY/max:0;let a=palettes[0],b=palettes.at(-1);for(let i=0;i<palettes.length-1;i++){if(p>=palettes[i].at&&p<=palettes[i+1].at){a=palettes[i];b=palettes[i+1];break}}const t=(p-a.at)/(b.at-a.at||1);names.forEach((n,i)=>document.documentElement.style.setProperty(n,mix(a.c[i],b.c[i],t)));document.documentElement.style.setProperty('--star-opacity',a.stars+(b.stars-a.stars)*t)}
let ticking=false;addEventListener('scroll',()=>{if(!ticking){requestAnimationFrame(()=>{updateSky();ticking=false});ticking=true}},{passive:true});updateSky();

const starField=document.querySelector('.stars');
if(starField){const stars=document.createDocumentFragment();for(let i=0;i<96;i+=1){const star=document.createElement('i');const size=(Math.random()*1.5+.55).toFixed(2);star.style.setProperty('--star-size',`${size}px`);star.style.setProperty('--star-x',`${(Math.random()*100).toFixed(2)}%`);star.style.setProperty('--star-y',`${(Math.random()*100).toFixed(2)}%`);star.style.setProperty('--star-alpha',(Math.random()*.55+.28).toFixed(2));star.style.setProperty('--star-speed',`${(Math.random()*3+2.4).toFixed(2)}s`);star.style.setProperty('--star-delay',`${(-Math.random()*5).toFixed(2)}s`);stars.appendChild(star)}starField.appendChild(stars)}

const breakScreen=document.querySelector('#breakScreen');const counter=document.querySelector('#cursorCountdown');const desktop=document.querySelector('.desktop-demo');let demoTimer;let autoPlayed=false;let lastPointer={x:0,y:0};
function hideStage(){clearInterval(demoTimer);breakScreen.classList.remove('active');counter.hidden=true;document.querySelectorAll('.nudge').forEach(n=>n.classList.remove('show'))}
function showBreak(){hideStage();breakScreen.classList.add('active')}
function placeCounter(){const r=desktop.getBoundingClientRect();counter.style.left=`${Math.max(16,Math.min(r.width-52,lastPointer.x+14))}px`;counter.style.top=`${Math.max(16,Math.min(r.height-70,lastPointer.y+18))}px`}
function startCountdown(){if(autoPlayed)return;autoPlayed=true;hideStage();let n=5;counter.textContent=n;counter.hidden=false;placeCounter();demoTimer=setInterval(()=>{n-=1;if(n>0){counter.textContent=n}else{showBreak()}},850)}
desktop?.addEventListener('pointermove',e=>{const r=desktop.getBoundingClientRect();lastPointer={x:e.clientX-r.left,y:e.clientY-r.top};if(!autoPlayed)startCountdown();else if(!counter.hidden)placeCounter()});
desktop?.addEventListener('pointerenter',e=>{const r=desktop.getBoundingClientRect();lastPointer={x:e.clientX-r.left,y:e.clientY-r.top};startCountdown()},{once:true});
document.querySelector('[data-show-break]')?.addEventListener('click',showBreak);
document.querySelectorAll('[data-nudge]').forEach(button=>button.addEventListener('click',()=>{hideStage();document.querySelector(`.nudge-${button.dataset.nudge}`)?.classList.add('show')}));
setTimeout(()=>{if(!autoPlayed&&desktop){lastPointer={x:desktop.clientWidth*.58,y:desktop.clientHeight*.48};startCountdown()}},700);

document.querySelectorAll('[data-science-tab]').forEach(button=>{
  button.addEventListener('click',()=>{
    const key=button.dataset.scienceTab;
    document.querySelectorAll('[data-science-tab]').forEach(tab=>{const on=tab===button;tab.classList.toggle('active',on);tab.setAttribute('aria-selected',on)});
    document.querySelectorAll('[data-science-panel]').forEach(panel=>panel.classList.toggle('active',panel.dataset.sciencePanel===key));
  });
});

let intentionalBlinks=0;
document.querySelector('#blinkNow')?.addEventListener('click',()=>{
  const lab=document.querySelector('.blink-lab');intentionalBlinks+=1;
  document.querySelector('#blinkCount').textContent=`${intentionalBlinks} intentional blink${intentionalBlinks===1?'':'s'}`;
  lab.classList.remove('blinking');void lab.offsetWidth;lab.classList.add('blinking');
});

const distanceRange=document.querySelector('#distanceRange');
function updateDistance(){const inches=Number(distanceRange.value);document.querySelector('#distanceValue').textContent=`${inches} in`;document.querySelector('#distanceStatus').textContent=inches<20?'Bring the monitor farther away.':inches>28?'Comfortable, if the text remains easy to read.':'A comfortable viewing distance.'}
distanceRange?.addEventListener('input',updateDistance);updateDistance();

let resetTimer;
document.querySelector('#startReset')?.addEventListener('click',()=>{
  clearInterval(resetTimer);let left=20;const output=document.querySelector('#resetTime');const prompt=document.querySelector('#resetPrompt');output.textContent=left;prompt.textContent='Look beyond the screen.';
  resetTimer=setInterval(()=>{left-=1;output.textContent=left;if(left<=0){clearInterval(resetTimer);prompt.textContent='That was the whole reset.'}},1000);
});

const hoursRange=document.querySelector('#screenHours');
function updateScreenMath(){const hours=Number(hoursRange.value);const cycles=Math.floor(hours*60/20);const longBreaks=Math.floor(cycles/5);const shortBreaks=cycles-longBreaks;const recoveryMinutes=Math.round((longBreaks*180+shortBreaks*25)/60);document.querySelector('#hoursOutput').textContent=`${hours} hour${hours===1?'':'s'}`;document.querySelector('#breakOutput').textContent=cycles;document.querySelector('#recoveryOutput').textContent=`${recoveryMinutes} min`}
hoursRange?.addEventListener('input',updateScreenMath);updateScreenMath();
