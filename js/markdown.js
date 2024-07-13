window.addEventListener("DOMContentLoaded", () => {
    // マークダウンの基本設定
    const markdown_setting = window.markdownit({
        html: true, // htmlタグを有効にする
        breaks: true, // md内の改行を<br>に変換
    });
    
    // .js-markdown-editerを定義
    const markdown_editer = $(".js-markdown-editer");
    
    // マークダウンの設定をjs-markdown-editerにHTMLとして反映させる
    const markdown_html = markdown_setting.render(getHtml(markdown_editer));
    markdown_editer.html(markdown_html);
        
    // 比較演算子（=，<>，<，<=，>，>=）をそのまま置換する
    function getHtml(selector) {
        let markdown_text = $(selector).html();
        markdown_text = markdown_text.replace(/&lt;/g, "<");
        markdown_text = markdown_text.replace(/&gt;/g, ">");
        markdown_text = markdown_text.replace(/&amp;/g, "&");
        console.log(markdown_text);
        return markdown_text;
    }
});
