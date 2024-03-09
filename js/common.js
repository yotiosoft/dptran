//共通パーツ読み込み
$(function() {
    /*if (window.innerWidth <= 767　|| navigator.userAgent.match(/iPhone|Android.+Mobile/)) {
        $("#header").load("/header_m.html");
        $("#footer").load("/footer_m.html");
    }
    else {*/
        $("#header").load("./header.html");
        $("#footer").load("./footer.html");
    //}
    
    /*window.addEventListener('resize', function(){
        if (window.innerWidth <= 767　|| navigator.userAgent.match(/iPhone|Android.+Mobile/)) {
            $("#header").load("/header_m.html");
            $("#footer").load("/footer_m.html");
        }
        else {
            $("#header").load("/header.html");
            $("#footer").load("/footer.html");
        }
    });*/
});
