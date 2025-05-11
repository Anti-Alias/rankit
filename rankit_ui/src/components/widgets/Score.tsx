import './Score.css';

export function Score({ winRate }: { winRate: number }) {
  const color = scoreColor(winRate);
  const winPercent = Math.round(winRate * 100); 
  return (
    <div className="Score" style={{backgroundColor: color}}>
      {winPercent}% 
    </div>
  )
} 

interface Rgb { r: number, g: number, b: number };
const rgbBest   = { r: 0,   g: 150, b: 60 };
const rgbWorst  = { r: 150, g: 50,  b: 50 };

function scoreColor(winRate: number): string {
  const color: Rgb = {
    r: rgbWorst.r + (rgbBest.r - rgbWorst.r) * winRate,
    g: rgbWorst.g + (rgbBest.g - rgbWorst.g) * winRate,
    b: rgbWorst.b + (rgbBest.b - rgbWorst.b) * winRate,
  };
  return `rgb(${color.r}, ${color.g}, ${color.b})`;
}
