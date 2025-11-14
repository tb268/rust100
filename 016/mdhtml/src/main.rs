use std::fs::File;
use std::io::BufReader;
use std::io::Write;
use std::io::BufRead;

fn convert_markdown_to_html(markdown: &str) -> String {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n<meta charset=\"UTF-8\">\n<title>Markdown to HTML</title>\n</head>\n<body>\n");
    let mut in_list = false;
    
    for line in &lines {
        let trimmed = line.trim();
        
        // 空行の処理
        if trimmed.is_empty() {
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            } else {
                html.push_str("<br>\n");
            }
            continue;
        }
        
        // 見出しの処理
        if trimmed.starts_with("### ") {
            let content = convert_inline_formatting(&trimmed[4..]);
            html.push_str(&format!("<h3>{}</h3>\n", content));
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
        } else if trimmed.starts_with("## ") {
            let content = convert_inline_formatting(&trimmed[3..]);
            html.push_str(&format!("<h2>{}</h2>\n", content));
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
        } else if trimmed.starts_with("# ") {
            let content = convert_inline_formatting(&trimmed[2..]);
            html.push_str(&format!("<h1>{}</h1>\n", content));
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
        } else if trimmed.starts_with("* ") {
            // リストアイテムの処理
            if !in_list {
                html.push_str("<ul>\n");
                in_list = true;
            }
            let content = convert_inline_formatting(&trimmed[2..]);
            html.push_str(&format!("<li>{}</li>\n", content));
        } else {
            // 段落の処理
            if in_list {
                html.push_str("</ul>\n");
                in_list = false;
            }
            let content = convert_inline_formatting(trimmed);
            html.push_str(&format!("<p>{}</p>\n", content));
        }
    }
    
    if in_list {
        html.push_str("</ul>\n");
    }
    
    html.push_str("</body>\n</html>");
    html
}

fn convert_inline_formatting(text: &str) -> String {
    let mut result = String::new();
    let mut chars = text.chars().peekable();
    
    while let Some(ch) = chars.next() {
        if ch == '*' {
            if let Some(&next) = chars.peek() {
                if next == '*' {
                    // 太字の処理 **text**
                    chars.next(); // 次の * を消費
                    let mut bold_text = String::new();
                    let mut found_end = false;
                    
                    while let Some(&ch) = chars.peek() {
                        if ch == '*' {
                            chars.next();
                            if chars.peek() == Some(&'*') {
                                chars.next();
                                result.push_str(&format!("<strong>{}</strong>", bold_text));
                                found_end = true;
                                break;
                            } else {
                                bold_text.push('*');
                                bold_text.push(ch);
                            }
                        } else {
                            bold_text.push(ch);
                            chars.next();
                        }
                    }
                    
                    if !found_end {
                        result.push_str("**");
                        result.push_str(&bold_text);
                    }
                } else {
                    // イタリックの処理 *text*
                    let mut italic_text = String::new();
                    let mut found_end = false;
                    
                    while let Some(&ch) = chars.peek() {
                        if ch == '*' {
                            chars.next();
                            result.push_str(&format!("<em>{}</em>", italic_text));
                            found_end = true;
                            break;
                        } else {
                            italic_text.push(ch);
                            chars.next();
                        }
                    }
                    
                    if !found_end {
                        result.push('*');
                        result.push_str(&italic_text);
                    }
                }
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }
    
    result
}

fn main() {
    let file_path = "input.md";
    let output_path = "output.html";
    
    // Markdownファイルを読み込む
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);
    let mut markdown_content = String::new();
    
    for line in reader.lines() {
        let line = line.unwrap();
        markdown_content.push_str(&line);
        markdown_content.push('\n');
    }
    
    // HTMLに変換
    let html_content = convert_markdown_to_html(&markdown_content);
    
    // HTMLファイルに書き込む
    let mut output = File::create(output_path).unwrap();
    output.write_all(html_content.as_bytes()).unwrap();
    
    println!("変換が完了しました: {} -> {}", file_path, output_path);
}
