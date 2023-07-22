var canvas = document.getElementById('canvas');
        var context = canvas.getContext('2d');
        var W = window.innerWidth;
        var H = window.innerHeight;

        canvas.width = W;
        canvas.height = H;

        var fontSize = 16;
        var columns = Math.floor(W / fontSize);
        var drops = [];
        for(var i=0; i<columns; i++){
            drops.push(0);
        }
        var str = "0123456789abcdef()+=/![]?<>;:*&^$#@";
        function draw(){
            context.fillStyle = "rgba(1,1,1,0.067)";
            context.fillRect(0, 0, W, H);
            context.font = "300 " + fontSize + "px fira code";
            context.fillStyle = "#49FFFC";
            for(var i=0; i<columns; i++){
                var index = Math.floor(Math.random()*str.length);
                var x = i * fontSize;
                var y = drops[i] * fontSize;
                context.fillText(str[index], x, y);
                if(y >= canvas.height && Math.random() > 0.99){
                    drops[i] = 0;
                }
                drops[i]++;
            }
        }
        draw();
        setInterval(draw, 40);

        window.addEventListener('resize', function(event) {
            W = window.innerWidth;
            H = window.innerHeight;

            canvas.width = W;
            canvas.height = H;

            columns = Math.floor(W / fontSize);
            drops = [];
            for(var i=0; i<columns; i++){
                drops.push(0);
            }
        }, true);

function getRandomColor() {
  var color = "#"
  var str = "49fffc 20fa5a eeeeee".split(" ")
  color += str[Math.floor(Math.random() * str.length)]
  return color;
}