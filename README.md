# the birthday project: com_music w/ rust

before we could go further, please read some introduction…

trước khi đến phần giải thích, tôi mong các bạn đọc thật kĩ phần mở đầu…

[https://open.spotify.com/track/51Mmmd7P1ETIqTTt2WtUrS?si=4fef7b1091cf45c7](https://open.spotify.com/track/51Mmmd7P1ETIqTTt2WtUrS?si=4fef7b1091cf45c7)

Trước khi viết ra những lời này, tôi muốn nói rằng đây là dự án mà tôi thực lòng để tâm nhất, chính xác hơn nữa, là dự án đầu tiên mà tôi tự làm sau nhiều năm với một tâm trạng thực sự khác và mang một kết quả cũng thực sự khác với trước đây. Với một tâm thế không vội vã, tôi dành thời gian cho một dự án mà không quá phức tạp về mặt cấu trúc, nhưng lại đủ khiến tôi căng não và trải nghiệm những cảm xúc khi làm ra một sản phẩm thực sự. Để đi ra khỏi cái phức cảm tự ti cá nhân, tôi đã rất cố gắng bằng nhiều cách, nào thì kiếm thành tích cao, nào thì cố gắng rải CV, nào thì xem mấy vid kiểu self-help,…. đủ thứ cả, nhưng cái cảm giác trống rỗng vẫn quẩn quanh trong lòng khiến tôi phát cáu. Đỉnh điểm của nỗi đau là trận ốm nặng vào cuối tháng 3 đã gần cướp đi mạng sống của tôi, đó là lần đầu tiên tôi thực sự hiểu rằng mình đã chịu đựng thứ cảm xúc tiêu cực ấy, tôi đã không thể yêu và chăm sóc bản thân cẩn thận. Tháng 4, tôi muốn tìm một lý do để chấm dứt cho những điều tệ hại ấy, mở ra một chương mà tôi đã bỏ dở và kết nối lại với bản thân mình cùng những ước mơ mà chính tôi đã phải bỏ lỡ khi trưởng thành. Có chút tham vọng, ước muốn, háo hức nhưng cũng có nỗi sợ, thất bại và đôi lúc là sự trì hoãn vì lo âu, đây có thể nói là một dự án mà mang khá nhiều cảm xúc của chính tôi, trong trạng thái mà tôi không còn bị tác động nhiều về mặt thời gian và tâm lý.

[the comeback 2024, com_music: gRPC x QUIC x FLAC](https://www.notion.so/the-comeback-2024-com_music-gRPC-x-QUIC-x-FLAC-ef191a3a19154d2995f62608b13155c6?pvs=21)

Thực ra dự án này tôi đã từng làm rồi, thậm chí nó còn là cơ sở cho phần mềm gọi điện mà tôi từng sáng tạo ra và được viết bằng Go. Có một điều lạ rằng tại sao tôi lại không bắt đầu bằng những dự án phức tạp như phần mềm quản lý hay game, thì tư duy của tôi đặt ra khi làm những phần mềm kiểu này là:

> đôi khi, những dự án lớn sẽ không có ý nghĩa nếu thiếu đi những dự án nhỏ hơn tạo nên nó.
> 

Tôi phải thừa nhận rằng các công ty hay các video dạy lập trình thường cố gắng vẽ ra một viễn tưởng về những phần mềm phức tạp, vè cái việc phần mềm bán ra sẽ được nghìn đô, theo cách khác, họ đánh đồng sự phức tạp kèm với năng lực. Tuy nhiên, quan điểm của tôi là năng lực không chỉ được đánh giá dựa trên việc con người có giải quyết được những vấn đề phức tạp hay không, mà là cách giải quyết vấn đề đấy liệu có thực sự hiệu quả hay không. Điều đó có nghĩa là, năng lực là việc đơn giản hoá sự phức tạp, chứ không phải sự phức tạp. Thế nên là, nếu có xem vài vid dạy lập trình cao siêu đấy, thì nên xem khi cần thôi, nó không phải là kinh thánh. Sự thật là, con người dần tiến tới sự tối giản trong mọi thứ, từ kiến trúc, thiết kế, cho đến những suy nghĩ bên trong, giống như việc người ta khi làm việc trong một đội nhóm, không thể cứ giải thích một vấn đề khó bằng những luân lý phức tạp, mà là phải giải thích và tối giản hoá những chi tiết ấy. Vốn năng lực của chúng ta không tập trung hết ở sự thông minh, mà là việc chúng ta có kiên trì theo đuổi một cái gì đó ý nghĩa, và dự án này nó mang hết tất cả sự kiên nhẫn, cố gắng và sự tối giản của tôi. Tôi chán với việc phải đánh giá bản thân mình theo sự áp đặt, mà tôi muốn bắt đầu bằng việc tôi kiên trì theo đuổi những ước mơ mà tôi đã bỏ lỡ nhiều năm. Mục tiêu của dự án này là vậy, một dự án không phức tạp về mặt cấu trúc, nhưng lại mang ý nghĩa về mặt tư duy, đưa một ai đó ra khỏi mặc cảm cá nhân, đi qua những cảm xúc để có thể làm một phần mềm thực sự chất lượng.

Để nói thêm, thì bài viết này sẽ giải thích sâu hơn những gì mà tôi viết trước đó, nếu như trước kia chỉ thuần code, thì các hàm lượng đều ở mức cân bằng, không chỉ có thuần code, vì không thể lập trình mà không hiểu được mình viết cái gì.

Tôi bắt đầu cảm nhận như thế đấy…. và đến lúc tôi bắt đầu bài phân tích của tôi :))))


# ii. the mechanism: how this software works???

work: giải thích chi tiết, từ lịch sử, liên hệ đến cơ chế hoạt động của phần mềm nghe nhạc, bóc tách thành phần và độc lập hoá các chi tiết ấy.

Ở phiên bản Go, tôi có viết nguyên lý hoạt động, tuy nhiên là chỉ thuần code do lúc đó rất vội và chưa hình thành cơ sở lý luận hoàn chỉnh mà chỉ dựa trên sự tương tự là chính. Điều này, với dân lập trình, có thể giúp họ bắt nhịp nhưng nếu lối làm việc này còn tiếp diễn, e là sẽ không có dự án thực sự nào cả. Điều này được chứng minh qua một khái niệm trong lập trình: “Tutorial Hell”. Thế nên, thay vì chỉ tập trung vào code như trước đó, tôi sẽ kể chi tiết hơn toàn bộ quá trình, ý tưởng và lựa chọn trong toàn bộ dự án này, để người đọc có một cái nhìn toàn cảnh về cách mà phần mềm thực sự chạy.

Và đây là link GitHub cho dự án:

[https://github.com/Bronstein0x113c1c3/com_music_with_rust](https://github.com/Bronstein0x113c1c3/com_music_with_rust)

## 1. technology choice.

Dự án trước, thực tế tôi đánh giá là có hiệu suất khá tốt, nếu không muốn nói là tốt nhất mà tôi đã có thể làm ra ở thời điểm đấy. Một dự án nảy nở trong thời kì vội vã, mang nỗi mặc cảm lớn và cảm giác thất bại lúc nào cũng có thể xảy đến, tôi biết ơn khi biết mình đã cố gắng rất nhiều cho một dự án mà tôi còn chẳng biết nguyên lý từ đầu và phải vực dậy bản thân để có thể tìm hiểu sau những thất bại rất dài, từ việc lỗi trong triển khai dự án, cho tới việc không tương thích, rồi cả việc không kiểm soát được bộ nhớ và lỗi liên tục… Tóm lại là, dự án hoàn hảo trong chừng mực hoàn hảo của nó, tức là chưa thể tìm ra một giải pháp tốt hơn. Thế nhưng, tôi nhận ra ở cái hiện tại này, mình có thể làm dự án đấy một lần nữa, nhưng sẽ thật chán nếu như cứ chỉ là những gì mình biết rồi, và nó lại trở nên vô dụng nữa…, từ đó, có một vài ý tưởng như sau:

- Độ an toàn bộ nhớ: Tôi cần một server mà hoạt động của nó không bị ảnh hưởng bởi vấn đề bộ nhớ gây ra, ví dụ là khi một client thoát ra, thì các thông tin về client khác vẫn chạy bình thường. Thêm nữa, server ấy phải chạy thật tối ưu, không ngốn tài nguyên nhưng phải đạt đến mức hoàn hảo, thống kê bộ nhớ phải thẳng tắp do không allocate bộ nhớ quá nhiều.
- Độ trễ truyền tải thấp: Một người muốn nghe nhạc thì không nên chờ quá lâu để bài nhạc đấy ra lò. Thực tế thì, dự án trước đó của tôi cũng đã phần nào giải quyết được độ trễ do chọn đúng ngôn ngữ lập trình, nhưng việc phải tải nguyên bài hát cũng đã khiến cho chương trình chậm đi rất nhiều. Độ trễ cao, cùng với vấn đề bộ nhớ đã khiến tôi phải suy nghĩ rất nhiều đến việc cải tổ toàn bộ chương trình. Vậy nên, thử thách của dự án ở mức cao hơn trước là, ngoài việc nghe được đầy đủ bản nhạc, còn phải đảm bảo độ trễ thấp, không để cho người nghe chờ đợi quá lâu cho một đoạn nhạc nào đó.
- Tính giáo dục: Một dự án lập trình, bằng cả tâm trí, không thể kết thúc mà không để lại một bài học hay một ý tưởng sau này. Lập trình là thứ luôn thay đổi qua năm tháng, con người liên tục phải học. Chính vì vậy, thay vì cố gắng cố định vào một tư duy trong một ngôn ngữ cũ, việc tận dụng một ngôn ngữ mới và làm chủ nó sẽ giúp người học hiểu ra tư duy và tự chủ hơn trong tương lai.

Từ ba ý định đó, thì tôi chọn Rust - Tokio và UDP, đây là giải thích cho lựa chọn trên:

![image.png](the%20birthday%20project%20com_music%20w%20rust%201d6d9622fa1980feb9bfc8bfc1a85257/image%208.png)

- Rust có hiệu suất nhanh tương đương C, nhưng an toàn bộ nhớ, ít tiêu tốn tài nguyên do giảm thiểu việc cấp phát bộ nhớ. Rust sử dụng mô hình ownership, với tôn chỉ là tránh sử dụng bừa bãi bộ nhớ dẫn đến các lỗi liên quan như null pointer, race condition,… hơn nữa, lập trình bất đồng bộ trên Tokio cũng có cú pháp gần như Go nhưng hiệu quả nhờ phương pháp sử dụng bộ nhớ của Rust nên cho hiệu suất rất cao và ít tiêu tốn tài nguyên nhất có thể.
- UDP, mặc dù có nhược điểm rất lớn là không có cơ chế bảo vệ sự toàn vẹn của các gói tin, nhưng vì bản chất là connection-less, tức là không ràng buộc kết nối, dẫn đến việc UDP đảm bảo hiệu suất truyền gần như là tốt nhất, phù hợp để phát trực tiếp. Đi sâu xa hơn, thiết kế của HTTP/3 sau này cũng sử dụng UDP để giảm thiểu độ trễ do handshake. Các kỹ sư của Google cũng đi xa hơn bằng việc viết ra một kỹ thuật mới mà có tốc độ của UDP, nhưng vẫn đảm bảo tính toàn vẹn như TCP, đó là QUIC và sau này là HTTP/3, được sử dụng trong máy chủ của Cloudfare, Tencent, Microsoft,….

## 2. from an ordinary music player.

![Untitled Diagram.drawio.svg](the%20birthday%20project%20com_music%20w%20rust%201d6d9622fa1980feb9bfc8bfc1a85257/Untitled_Diagram.drawio.svg)

Đây, chính là diagram cơ bản nhất của chương trình này dựa trên quy trình 3 bước đơn giản:

1. Biểu diễn âm thanh ra một dạng thức X
2. Lưu trữ các kết quả của dạng thức X, có thể mang đi, gói ghém,….
3. Từ các dữ liệu của dạng thức X, phát sinh âm thanh trở lại.

Câu chuyện xuất phát đơn giản lắm, nó là một phần mềm nhạc phải phát được bài nhạc, nó phải phát được bài nhạc FLAC/MP3 ngay trong chính máy tính của chúng ta. Thế thì, dựa trên nguyên lý ba bước, thì chúng sẽ phải làm những việc như sau:

1. Phân giải file FLAC/MP3 sang dạng âm thanh, cụ thể là dạng 16 hoặc 24 bit, tốc độ mẫu là 44.1 kHz,… giống như có một thiết lập sẵn đại chúng ấy.
2. Lưu trữ hoặc vận chuyển các dạng byte, có thể lưu sang dạng bin, wav, pcm,……
3. Phân giải các đoạn dữ liệu thành các dạng âm thanh, ở đây là dạng âm thanh thô.

Với phần mềm này, tôi muốn nhấn mạnh cụm từ “vận chuyển” nhiều hơn là “lưu trữ”. Thực tế là, nếu phần mềm chỉ dừng lại ở lưu trữ, thì các phần mềm phát nhạc hiện đại không thể tồn tại. Hơn nữa, mục tiêu của phần mềm này là tạo ra một phần mềm nghe nhạc mà độ trễ là thấp nhất có thể, khác với mục tiêu ban đầu chỉ là phát nhạc. Ở đây, nếu đã là phần mềm nghe nhạc, thì nó không nên chỉ là một phần mềm chuyển file.

Vậy thì, điều đó yêu cầu phần mềm cho phía người dùng là phải làm đồng thời hai việc:

- Nhận các chunk từ phía máy chủ với độ trễ thấp nhất
- Từ các chunk nhận được, phát nhạc

Từ hai ý định trên, phần mềm này bắt buộc phải chạy đa luồng, chính xác hơn là concurrency. Đi xa hơn nữa, để có thể liên kết cả hai phần của client, bắt buộc phải dùng channel - sender và receiver, để tách ra hai phần của phần mềm nghe nhạc - điều này sẽ được giải thích ở phần triển khai và viết code.

Hơn hết, nếu nhạc và âm thanh có thể biểu thị được dưới dạng dữ liệu thì bài toán này sẽ trở thành bài toán vận chuyển dữ liệu qua mạng, nhưng yêu cầu cao hơn là phải có độ trễ thấp và tính toàn vẹn dữ liệu cao - rất cao.

Toàn bộ ý tưởng và mô hình, thực chất là đã giải thích ở phần lý luận nên phần này tôi chỉ muốn đưa ra mục tiêu cho phần mềm. Những yếu tố này về sau sẽ được giải thích kĩ hơn ở phần triển khai và các giai đoạn tiến hoá.

## 3. before the implementation: potential and risk.

Mặc dù có mục tiêu là vậy, ý tưởng không tệ, nhưng rõ ràng phần mềm luôn có những vấn đề của nó trước khi triển khai, đây cũng vậy. Ở đây, tôi muốn nhắc về tiềm năng phát triển của nó cũng như một vài vấn đề gặp phải trước, trong khi và sau khi phát triển.

Phải nói rằng, ý tưởng chuyển sang UDP là tuyệt vời và đúng đắn nhất, mặc dù không hoàn hảo. Để đảm bảo việc truyền tải không có độ trễ và chỉ tập trung vào đó, việc chuyển từ TCP sang UDP là điều tất yếu. Mục tiêu là giảm độ trễ do ACK cũng như RTT, thêm nữa, nếu như có một gói tin bị mất thì toàn bộ quá trình vận chuyển của phần mềm sẽ không bị gián đoạn. Có thể hiểu là, phần mềm phát nhạc này cần độ toàn vẹn, nhưng chưa đến mức như phần mềm chuyển file nên có thể chấp nhận mất mát một ít, nhưng phải đảm bảo quá trình phát nhạc thông suốt. Trước phiên bản bằng Rust với UDP, tôi đã từng làm bằng Go - gRPC - HTTP/3 và có kết quả khả quan, cũng như có một phiên bản bằng Rust nhưng dùng TCP và JSON thì dù bằng cách nào, tôi vẫn phải cố nhịn chuyển một file JSON nặng mà không thể tách ra toàn bộ các byte để tập trung vận chuyển. Phải đến khi tôi lật tung bản đầu tiên thì tôi mới khám phá ra và sau đó là toàn bộ lý luận ở phần lịch sử. Mặc dù phần tiềm năng không nhiều, nhưng đây lại là lý do cốt tuỷ và quan trọng nhất, mặc dù sau đó liên tiếp những vấn đề mà tôi phải đối mặt.

Rõ ràng, một chương trình không bao giờ trở nên quá hoàn hảo mà hơn hết, nó cũng có vấn đề nghiêm trọng, thậm chí là phải đến khi tôi sắp xong chương trình thì toàn bộ vấn đề mới bộc lộ. Việc đi khỏi TCP cũng khiến cho cách kiểm soát của chương trình phải viết lại từ đầu, từ việc nhận gói tin, điều phối kết nối cho đến việc tối ưu lại các tác vụ để sao cho chúng đạt được hiệu quả về mặt điều phối cũng như truyền tải. Điều tệ hơn là, việc không sử dụng TCP khiến chương trình phức tạp về mặt cấu trúc, cũng như hiện tượng packet loss xảy ra (biểu hiện ở dấu hiệu nhạc không đủ dữ liệu và nghe như tình trạng xước đĩa). Nó giống như là, bạn phải làm tất cả mọi thứ từ đầu mà không có một sự trợ giúp đàng hoàng. Nó là khó, rất khó và siêu khó khi phải bắt đầu lại mà không có một sự trợ giúp nào cả….

Điều xa nhất là, tôi coi phần mềm này giống như một canh bạc vậy, không phải vì nó phức tạp, mà là việc phải bắt đầu lại. Phần lớn thời gian, tôi chỉ dành cho việc quản lý kết nối nên thực sự, nó rất khó cho tôi. Chương trình, mặc dù phần cấu trúc thì dễ hiểu, nhưng thực sự triển khai nó là một cái gì đấy căng não. Gần như là mỗi lần chương trình bị lỗi cái gì đấy, là tôi mất nguyên tiếng để sửa chữa và test đi test lại. Cay đắng hơn là chương trình này phải chạy chay mới khám ra được lỗi thế nên nó cần sự kiên nhẫn. Và đến cuối cùng, khi chương trình gần xong và chuẩn bị tiết mục phát nhạc, vấn đề packet loss lại đến như một gáo nước lạnh khiến cho dự án suýt toang. Ít nhất là, vấn đề được giải quyết trước hôm sinh nhật nên tôi cũng thở phào và cũng định đóng gói dự án….

Tuy nhiên, sự xuất hiện của QUIC gần như làm thay đổi mọi thứ. QUIC cho tốc độ tương đương với UDP thuần mà vẫn giữ khả năng điều phối của TCP. Tuy nhiên, do không còn quá nhiều thời gian và tôi muốn phần mềm này về đúng bản chất là demo ý tưởng, nên sau này, tôi sẽ viết lại style của dự án dùng QUIC, cũng như QUIC sẽ được sử dụng rộng rãi trong các dự án sau này của tôi.

Với tôi thì, dự án này không phức tạp về mặt cấu trúc, cũng chỉ là việc điều phối kết nối, nhưng điều thực sự khó là, cái cảm giác thất bại trực chờ mỗi khi tôi gặp khó, nhiều lúc khiến tôi muốn dừng lại, nhưng tôi vẫn đi tiếp và tinh thần cho dự án sau này của tôi cũng sẽ như vậy, là việc tôi sẽ đối mặt với những khó khăn và thách thức. Dù thực sự rất khó, nhưng tôi phần nào hãnh diện vì cũng kịp làm một phần mềm mà phải bắt đầu bằng tay không, đủ để khiến tôi cảm nhận được niềm vui từ nó.

# iii. implementation: opportunities and challenges

[https://open.spotify.com/album/6Lq1lrCfkpxKa4jCo5gKWr?si=Rrjvstp_QYiA-XAykejOMQ](https://open.spotify.com/album/6Lq1lrCfkpxKa4jCo5gKWr?si=Rrjvstp_QYiA-XAykejOMQ)

Ở phần này, tôi sẽ trình ra các đoạn code và giải thích cơ chế hoạt động. Thực tế thì, các lý luận của tôi về âm thanh và cơ chế hoạt động cũng dựa phần lớn vào các đoạn code, đặc biệt là sau khi tôi phân tích dự án này từ đầu với bản Go cũng như việc sang UDP càng làm nổi bật phần lý luận cũng như các đoạn code sau đó. Đây, có thể nói là toàn bộ quá trình làm, một cách chi tiết nhất. Tôi có thể gọi toàn bộ quá trình này là quá trình tiến hoá của toàn bộ dự án, là vì tư duy của dự án này đi xa hơn so với những gì mà ta có thể nhìn thấy. Kèm theo đó, tôi cũng sẽ nói một vài vấn đề và bất cập của chương trình. Sự thật là chương trình của tôi cần sửa rất nhiều và cần nhiều đợt tiến hoá, nhưng ít nhất, nó cũng đã đi được những bước đi đáng kể, chỉ trong 2 tuần ngắn ngủi để đưa một dự án nằm mộ tận một năm trở lại. 

it’s not about the perfection of a software, but the process, both the success and the failure, to make a man.

## part 1: the experimental.

Bây giờ, trước khi tôi đến phần experimental - cái phần mà tôi đưa ra tất cả các thử nghiệm cũng như ý tưởng đầu tiên, tôi sẽ phải trình diện phần code TCP của phiên bản đầu tiên….

- first edition
    
    ```rust
    use rodio::{source::Source, Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    
    fn main() {
        // Get a output stream handle to the default physical sound device
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    
        // Load a sound from a file
        let file = BufReader::new(File::open("rightforyou.mp3").unwrap());
    
        // Decode that sound file into a source
        let mut source = Decoder::new(file).unwrap();
    
        // Get the sample rate and the number of channels of the source
        let sample_rate = source.sample_rate();
        let channels = source.channels();
        let mut sink = Sink::try_new(&stream_handle).unwrap();
    
        //enough intro!!!
    
        use std::sync::{Arc, Mutex};
        use std::thread;
    
        let buffer: Vec<i16> = source.by_ref().collect();
        let buffer = Arc::new(Mutex::new(buffer));
    
        let sink = Arc::new(Mutex::new(sink));
    
        let sink_clone = Arc::clone(&sink);
        let buffer_clone = Arc::clone(&buffer);
    
        thread::spawn(move || {
            let binding = buffer_clone.lock().unwrap();
            let mut iter = binding.chunks(2000);
            loop {
                let chunk = match iter.next() {
                    Some(chunk) => chunk.to_vec(),
                    None => break,
                };
    
                let new_source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, chunk);
    
                let mut sink = sink_clone.lock().unwrap();
                sink.append(new_source);
            }
        }).join();
    
        sink.lock().unwrap().sleep_until_end();
    
        // Create a new source from the buffer
        // let source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, buffer);
    
        // // Play the sound directly on the device
        // stream_handle.play_raw(source.convert_samples());
    
        // The sound plays in a separate audio thread,
        // so we need to keep the main thread alive while it's playing.
    }
    ```
    
- TCP server
    
    ```rust
    use std::{io::Write, net, thread};
    use std::io::{BufReader};
    use std::fs::File;
    use rodio::{source::Source, Decoder, OutputStream, Sink};
    use serde_json::json;
        
    fn handle_conn(conn: &mut net::TcpStream) {
        println!("connected to client, {}", conn.local_addr().unwrap());
        let file = BufReader::new(File::open("04 - In My Blood.flac").unwrap());
        /*
        Audio need 3 things:
            1. buffers/payload.
            2. sample rate
            3. channels
        */
        println!("getting file done, start extracting to info");
        
        let mut source = Decoder::new(file).unwrap();
        let buffer: Vec<i16> = source.by_ref().collect();
        let sample_rate = source.sample_rate();
        let channels = source.channels();
    
        println!("done getting info, start transferring");
        
        //done gathering info
    
        let res = json!({
            "payload": buffer,
            "sample_rate": sample_rate,
            "channels":channels,
        });
        conn.write_all(res.to_string().as_bytes());
        conn.flush();
        println!("transferring done, happy listening for client!");
    
        conn.shutdown(net::Shutdown::Both).unwrap();
    }
    fn main() {
        let listener = net::TcpListener::bind("127.0.0.1:8080").unwrap();
        for conn in listener.incoming() {
            let mut conn = conn.unwrap();
            thread::spawn(move || handle_conn(&mut conn));
        }
    }
    ```
    
- TCP client
    
    ```rust
    use serde::Deserialize;
    use serde_json::Result;
    use rodio::{source::Source, Decoder, OutputStream, Sink};
    #[derive(Deserialize, Debug)]
    struct Audio {
        payload: Vec<i16>,
        sample_rate: u32,
        channels: u16,
    }
    
    fn process_audio(audio: Audio){
        use std::sync::{Arc, Mutex};
        use std::thread;
    
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let mut sink = Sink::try_new(&stream_handle).unwrap();
        
        let sample_rate = audio.sample_rate;
        let channels = audio.channels; 
    
        println!("gathering up info done, starting setting to hear...");
    
        let buffer = Arc::new(Mutex::new(audio.payload));
        let sink = Arc::new(Mutex::new(sink));
        let sink_clone = Arc::clone(&sink);
        let buffer_clone = Arc::clone(&buffer);
    
        println!("everything done! happy listening");
    
        thread::spawn(move || {
            let binding = buffer_clone.lock().unwrap();
            let mut iter = binding.chunks(2000);
            loop {
                let chunk = match iter.next() {
                    Some(chunk) => chunk.to_vec(),
                    None => break,
                };
    
                let new_source = rodio::buffer::SamplesBuffer::new(channels, sample_rate, chunk);
    
                let mut sink = sink_clone.lock().unwrap();
                sink.append(new_source);
            }
        }).join();
    
        sink.lock().unwrap().sleep_until_end();
    
    }
    
    fn main() -> Result<()> {
        let mut conn = net::TcpStream::connect("127.0.0.1:8080").unwrap();
        println!("{}","connected, starting downloading the sound.....");
        let mut res = String::new();
        conn.read_to_string(&mut res).unwrap();
        // let data = r#"{"numbers": [1, 2, 3, 4, 5]}"#;
        println!("{}","sound downloaded, extracting....");
    
        let data = res.as_str();
    
        let deserialized: Audio = serde_json::from_str(data)?;
    
        // println!("{:?}", deserialized.channels);
    
        conn.shutdown(net::Shutdown::Both).unwrap();
        println!("{}","extracting done, setting things up");
    
        process_audio(deserialized);
        Ok(())
    }
    ```
    

Đây là toàn bộ phần đầu tiên làm nền cho phần experimental, các bạn có thể thấy là ý tưởng cũng đã thành hình và tôi cũng tách được toàn bộ chương trình rồi. Thế nhưng mà, có một vài điểm lưu ý sau đây:

- Các bản trên sử dụng TCP: Mặc dù đảm bảo tốt tính toàn vẹn, nhưng sẽ gây khó khăn cho việc xử lý nhạc và độ trễ của chương trình tăng lên gấp bội. Có thể nói đây là lý do chí mạng khiến tôi bỏ TCP và sang UDP. Sau đó, cũng ở phần 2 và 3, vấn đề điều phối kết nối và packet loss cũng được đưa ra, cho thấy những bất lợi của UDP, cũng như sự chuyển dịch sang QUIC ở phần 4 đã giải quyết tốt tình trạng này ra sao.
- Sử dụng thread thay vì async thread/green thread:
    - Các thread yêu cầu cấp phát bộ nhớ quá nhiều mà không có một cơ chế để điều phối: Mỗi thread ăn cố định 2MB stack, không có một cơ chế tạm thời dừng thực thi và cho ngủ, dẫn đến mức độ ăn bộ nhớ có thể lên tới tầm GB, hoặc TB là chuyện hoàn toàn có thể xảy ra.
    - Việc không có một cơ chế điều phối, tạm nghỉ thực thi khiến thread không thực sự hợp với các chương trình nặng về trao đổi dữ liệu - IO bound. Với các chương trình nặng về trao đổi dữ liệu, thì vốn dĩ chúng sẽ phải chờ rất lâu cho một tín hiệu dữ liệu nào đó, và điều này sẽ gây lãng phí tài nguyên cho CPU, do đó, phải có một cơ chế điều phối kết nối và không gì khác, chỉ có phương pháp kết hợp giữa bất đồng bộ và đa luồng - vừa giải quyết bài toán trao đổi dữ liệu, vừa giải quyết các tác vụ liên quan đến CPU.
- JSON: Đó là một ý tưởng cực kì ngu dốt mà tôi đã tạo ra, nhìn lại thì phải nói là vậy. Trong lúc túng quẫn, tôi đã ném sạch toàn bộ dữ liệu vào một ổ JSON, và nó nặng càng nặng. Việc ném vào ổ JSON đã khiến cho máy chủ trở nên quá tải, chậm đi, mà client cũng không khác là bao. Vốn dĩ, mục tiêu của tôi là biến file nhạc thành các đoạn có thể truyền tải thô qua mạng, chứ không phải là ném cả một đống dữ liệu vô ích. Điều này chính tôi phải chỉnh sửa sau này khi tiến hành ở phần 3.

Đó là 3 vấn đề chính và bắt đầu, tôi phải thấu hiểu toàn bộ bản chất hoạt động của phần mềm hơn là việc sử dụng quá nhiều AI để code hộ. Xin nói luôn là toàn bộ phần experimental là do tôi đọc sách về Rust, cũng như tìm hiểu nguyên lý bất đồng bộ và tham khảo từ bản dự án Go đầu tiên.

Và đây chính là phần experimental, do phần này rất dài nên tôi sẽ tách ra thành từng mảnh code riêng và bản hoàn chỉnh sẽ được gập lại để tiện đọc về sau:

### complete version of experimental.rs

```rust
use std::sync::{Arc, Mutex};

use tokio::{self, sync::mpsc::unbounded_channel};

use rodio::{Decoder, OutputStream, Sink};
#[tokio::main]
async fn main() {
    // use tokio::io::{self, AsyncBufReadExt};

    // let stdin = io::stdin();
    // let mut reader = io::BufReader::new(stdin).lines();

    // println!("Type something and press Enter (Ctrl+D to exit):");

    // while let Ok(Some(line)) = reader.next_line().await {
    //     println!("You typed: {}", line);
    // }

    /* */
    let (stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    // Add a oneshot channel for completion notification
    let (complete_tx, complete_rx) = tokio::sync::oneshot::channel();

    let file = std::io::BufReader::new(
        std::fs::File::open("./list_songs/rightforyou_instrumental.mp3").unwrap(),
    );
    let mut source = Decoder::new(file).unwrap();

    // Producer task
    let t = tokio::spawn(async move {
        let buffer: Vec<i16> = source.by_ref().collect();
        let bytes: Vec<u8> = buffer
            .iter()
            .flat_map(|&num| num.to_be_bytes().to_vec())
            .collect();
        let mut iter = bytes.chunks(16384);

        while let Some(res) = iter.next() {
            let mut salt = "music_chunk".as_bytes().to_vec();
            let mut res = res.to_owned();
            res.append(&mut salt);
            tx.send(res);
        }

        // Signal that we've finished sending all data
    });

    // Consumer task
    let s1 = sink.clone();
    let t3 = tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            println!("receiving chunk...");
            let res: Vec<i16> = res[..res.len() - 11]
                .chunks_exact(2)
                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
            s1.lock().unwrap().append(res);
        }
        complete_tx.send(()).unwrap();
    });

    // Player wait task
    let t2 = tokio::spawn(async move {
        // Wait for either:
        // 1. The completion signal (all data sent)
        // 2. The consumer task to finish (in case of error)
        tokio::select! {
            _ = complete_rx => {
                println!("All data received, waiting for playback to finish...");
                sink.lock().unwrap().sleep_until_end();
                println!("Playback completed!");
            }
            // _ = t3 => {
            //     println!("Consumer task finished early");
            // }
        }
    });

    // Wait for all tasks to complete
    tokio::try_join!(t, t3, t2).unwrap();
}

#[tokio::test]
async fn comparing() {
    let mut vec1 = Vec::with_capacity(300);
    let mut vec2 = vec![4, 5, 6, 8];

    {
        vec1 = vec2;
    }
    // (&mut vec1) = vec2;
    println!("{:?}", vec1.len());
}

#[tokio::test]
async fn test_equal() {
    let test_str = "done".as_bytes().to_vec();
    assert!(&test_str == "done".as_bytes());
}

#[tokio::test]
async fn test_some() {
    let buffer: Vec<i16> = vec![-1, 34, 56, 23, 32535, 0];
    let expected: Vec<u8> = vec![255, 255, 0, 34, 0, 56, 0, 23, 127, 23, 0, 0];
    let bytes: Vec<u8> = buffer
        .iter()
        .flat_map(|&num| num.to_be_bytes().to_vec()) // little-endian
        // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
        .collect();
    // assert!(bytes,"{}", expected);
    for i in 0..expected.len() {
        assert!(expected[i] == bytes[i], "done!!");
    }
    // println!("{:?}", bytes);
    // println!("{:?}", expected);
}

#[tokio::test]

async fn test_transferring() {
    // println!("{}",str);
    let (stream, stream_handle) = OutputStream::try_default().unwrap();

    let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
    let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
    let t = tokio::spawn(async move {
        let file = std::io::BufReader::new(
            std::fs::File::open("./list_songs/02 - Good To Be.flac").unwrap(),
        );
        let mut source = Decoder::new(file).unwrap();

        // source.
        let buffer: Vec<i16> = source.by_ref().collect();
        // println!("{:?}", &buffer);
        let bytes: Vec<u8> = buffer
            .iter()
            .flat_map(|&num| num.to_be_bytes().to_vec()) // little-endian
            // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
            .collect();
        let mut iter = bytes.chunks(2048); // 1mb per chunk....

        while let Some(res) = iter.next() {
            tx.send(res.to_owned());
        }
    });
    let s1 = sink.clone();

    let t3 = tokio::spawn(async move {
        while let Some(res) = rx.recv().await {
            // socket.send_to(&res, addr).await.unwrap();
            // println!("the receiver: {:?}",res);
            // assert!()
            println!("receiving chunk...");
            let res: Vec<i16> = res
                .chunks_exact(2)
                .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                .collect();
            let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
            s1.lock().unwrap().append(res);
        }
    });
    let t2 = tokio::task::spawn(async move {
        sink.lock().unwrap().sleep_until_end();
        println!("waiting is called!!!!");
        // tokio::task::yield_now().await;
        println!("now, yield...");
        println!("the player has been done!!!!")
    });

    t.await;
    t2.await;
    t3.await;
}
#[tokio::test]
async fn test_eq() {
    let res = "abcxyz".to_string();
    assert!(&res[..3] != "abc", "they equals");
}

#[tokio::test]
async fn test_plus() {
    let mut i: u8 = 255;
    i = i + 1;
    assert!(i == 0);
}

#[tokio::test]
async fn test_close() {
    let (mut tx, mut rx) = unbounded_channel();
    let t1 = tokio::spawn(async move {
        while !tx.is_closed() {
            tx.send(1);
        }
        drop(tx);
    });

    for i in 0..3 {
        match rx.recv().await {
            Some(i) => {
                println!("{}", i);
            }
            None => {}
        }
    }
    rx.close();
    t1.await;
}
```

Câu chuyện bắt đầu…, bằng một buổi nghe nhạc nọ, chính xác hơn là từ dự án phần mềm nghe nhạc đầu tiên của tôi. Trên thực tế thì phần mềm đã thực sự thành công rồi, các bạn có thể thấy là tôi từng làm ra phiên bản TCP của nó rồi. Tuy nhiên, để bắt đầu trong trạng thái mình không thực sự biết gì, hay đúng hơn là, như mới, tôi sẽ phải tìm ra bản chất thực sự của chính phần mềm mà tôi làm, hơn là cứ bấu víu vào cái bản đầu tiên ấy.

Như kế hoạch ban đầu, phần mềm nghe nhạc có ba phần tác vụ làm việc độc lập và song song, khác với phiên bản đầu tiên:

1. Phân giải file nhạc hoặc nhận chunk từ phía server
2. Từ các dữ liệu được phân giải, đắp vào buffer của stream để phát nhạc từ lượng dữ liệu đó
3. Chốt chặn để phần mềm không kết thúc phát nhạc quá sớm, kịp thời dọn dẹp tài nguyên

Bản chất chương trình hoạt động là như vậy, tuy nhiên, từ một phiên bản chỉ dùng thread mà sang dạng async cần rất nhiều thời gian để nghiên cứu và chuyển đổi. Nhưng, điều diệu kỳ cũng đã xảy ra. Khi chuyển sang dạng async, điều này đã giúp cho chương trình linh hoạt hơn khá nhiều và đặc biệt, nó chạy vô cùng hoản hảo. Sự thực là, bản thân runtime - Tokio, không thực sự chạy đơn luồng mà tuỳ lúc, trình runtime có thể spawn thêm và sắp xếp lịch cho các tác vụ. Và đây là lời giải thích cho từng phần:

- part 1: mp3/flac → data, sending
    
    ```rust
    		let (stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Arc::new(Mutex::new(Sink::try_new(&stream_handle).unwrap()));
        let (mut tx, mut rx) = tokio::sync::mpsc::unbounded_channel();
        // Add a oneshot channel for completion notification
        let (complete_tx, complete_rx) = tokio::sync::oneshot::channel();
    
        let file = std::io::BufReader::new(
            std::fs::File::open("./list_songs/rightforyou_instrumental.mp3").unwrap(),
        );
        let mut source = Decoder::new(file).unwrap();
     // Producer task
        let t = tokio::spawn(async move {
            let buffer: Vec<i16> = source.by_ref().collect();
            let bytes: Vec<u8> = buffer
                .iter()
                .flat_map(|&num| num.to_be_bytes().to_vec())
                .collect();
            let mut iter = bytes.chunks(16384);
    
            while let Some(res) = iter.next() {
                let mut salt = "music_chunk".as_bytes().to_vec();
                let mut res = res.to_owned();
                res.append(&mut salt);
                tx.send(res);
            }
    
            // Signal that we've finished sending all data
        });
    ```
    
    Ở trong phần này, tôi tạo ra stream/buffer để tập trung phát âm thanh từ đoạn dữ liệu, bao gồm có stream_handle và sink. Đặc biệt, với sink, tôi phải dùng mutex và Arc (atomic reference counter) để cho phần 2 và phần 3 (phần nhận dữ liệu và phần kiểm soát trình phát). Tiếp đến, phân giải một đoạn nhạc, ở đây tôi chọn bài “Right For You” ưa thích của mình để tiến hành. Tôi cần 2 loại channel, một loại liên quan đến dữ liệu âm thanh và một loại là gủi tín hiệu khi không còn tín hiệu âm thanh.
    
    Kế hoạch rất đơn giản ở phía producer task - tác vụ phân giải dữ liệu nhạc, ở đây, tôi tách toàn bộ nhạc thành dạng 16 bit, có thể ở dạng có dấu hoặc không dấu, sau đó, lại phân giải dưới dạng 8 bit - bản chất của việc này là nếu sau đó, tôi có ý định (và rõ ràng là có) truyền phát qua mạng, thì việc đưa ra dạng 8 bit và tái tại thành công là việc bắt buộc. Tôi tạo ra một con trỏ iterator có độ rộng 16384 bytes, trượt qua toàn bộ đoạn dữ liệu. Trong quá trình trượt đấy, tôi gom lại dữ liệu và gửi nó qua channel, cái lệnh “**tx.send(res)**” đã chứng minh điều đó. Vốn dĩ, bản này có thể rất đơn giản nhưng ở phần 3, tôi phải lo vấn đề packet loss nên trong quá trình trượt, tôi đã phải chèn thêm một vài đoạn lạ để rồi sau đó có thể phân tách, đánh dấu rồi kiểm tra với mục đích để kiểm soát luồng dữ liệu - tránh mất mát dữ liệu quá nhiều do sử dụng UDP. Tuy vậy, có thể hiểu đơn giản là tôi phân tách thành từng đoạn âm thanh nhỏ rồi gửi từng đoạn âm thanh đó. Việc này có một vài mục đích:
    
    - Tối ưu băng thông truyền tải khi không cần phải gửi một đoạn dữ liệu quá lớn.
    - Có thể sử dụng dữ liệu ngay khi có dữ liệu âm thanh nhận được.
    - Tách chương trình và dễ dàng nâng cấp về sau
- part 2: from received data, fill the buffer.
    
    ```rust
      // Consumer task
        let s1 = sink.clone();
        let t3 = tokio::spawn(async move {
            while let Some(res) = rx.recv().await {
                println!("receiving chunk...");
                let res: Vec<i16> = res[..res.len() - 11]
                    .chunks_exact(2)
                    .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                    .collect();
                let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
                s1.lock().unwrap().append(res);
            }
            complete_tx.send(()).unwrap();
        });
    ```
    
    Ở phần này thì dễ hơn, tôi tập trung lấy dữ liệu được truyền qua từ trước đó, chuyển từ dạng 8 sang 16 bit và lấp đầy buffer bằng một sample rate mà tôi muốn, ở đây là 48kHz và 2 kênh. Đến khi toàn bộ dữ liệu được chuyển xong, tôi thông báo là kết thúc bài nhạc và dừng phát. Có thể nói rằng phần này đơn giản hơn là vì không phải tốn nhiều thời gian để chuẩn bị. Tuy nhiên, nếu nhìn rộng ra thì bản thân chương trình đã có thể tách ra thành phần mềm streaming nhạc qua mạng được rồi. Dẫu vậy, tôi vẫn cần phải có một minh chứng rằng chúng đã có thể tách ra, chỉ khi tôi tách công đoạn này thành công.
    
    Tóm lại là, phần này không phức tạp hơn phần đầu tiên mà chỉ là cách để lấy dữ liệu và ít nhất, đây bắt đầu cho toàn bộ quá trình sau đó.
    
- part 3: the patrol
    
    ```rust
     // Player wait task
        let t2 = tokio::spawn(async move {
            // Wait for either:
            // 1. The completion signal (all data sent)
            // 2. The consumer task to finish (in case of error)
            tokio::select! {
                _ = complete_rx => {
                    println!("All data received, waiting for playback to finish...");
                    sink.lock().unwrap().sleep_until_end();
                    println!("Playback completed!");
                }
                // _ = t3 => {
                //     println!("Consumer task finished early");
                // }
            }
        });
    
        // Wait for all tasks to complete
        tokio::try_join!(t, t3, t2).unwrap();
    ```
    
    Nhìn thì có vẻ không quan trọng, tuy nhiên, phần này là một trong số phần quái nhất mà tôi phải đối diện. Ở đây, nếu như tôi không có cơ chế báo hiệu, thì toàn bộ phần xử lý âm thanh sẽ bị drop, theo thuật ngữ của Rust thì đó là khi một vùng dữ liệu đã giải phóng và không còn sử dụng, dẫn đến việc chương trình nhạc bị kết thúc và không thể phát nhạc ngay sau đó. Đi xa hơn nữa, tôi cần được câu thời gian để cho chương trình phát nhạc xong, rồi lúc đó muốn kết thúc như nào thì tuỳ. Dẫu vậy, đây là phần khó nhằn nhất mà tôi phải xử lý, dù không mất quá nhiều dòng code.
    

Toàn bộ phần experimental này tôi làm trong vòng 2 ngày, vì thực tế là đã có cơ sở và hoàn toàn thành công. Đây cũng là bước đầu tiên tôi phá kén. Đặc biệt hơn, toàn bộ kiến thức về xử lý bất đồng bộ cũng hoàn toàn được thực hiện ở ngay phần experimental này, nên có thể nói rằng đây là dự án dạy tôi về áp dụng Rust cũng không sai và ít nhát tôi đã thành công. 

Đó chỉ là bước đầu tiên và mọi thứ còn đi xa hơn thế.

## part 2: UDP.

Ở phần UDP, tôi muốn tập trung riêng vào quá trình điều phối kết nối, một vài phân tích ban đầu cũng như vấn đề nội tại mà phải đến khi làm việc với QUIC thì mới khám phá dần. Mặc dù, ở phần UDP này, tôi có thể nói về packet loss nhưng mà tôi muốn dành riêng vì hầu hết thời gian cho dự án này nằm ở phần điều phối, cũng như việc dự án chênh vênh đến mức nào khi cứ một lần lỗi điều phối là phải mất cả tiếng để đọc lại code. Thế nên, mặc dù gọi là UDP, nhưng phần này tôi sẽ chỉ nói đến phần điều phối kết nối.

Phần code UDP này rất dài nên tôi sẽ phân tách code ra từng phần nhỏ hơn để dễ theo dõi.

### complete version in UDP control section.

```rust
//  song_processing;

// use crate::song_processing::something;
mod song_processing;
use rodio::Decoder;
use std::{
    collections::HashMap,
    net::SocketAddr,
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{
    net::UdpSocket,
    select,
    sync::{
        Mutex,
        mpsc::{self, UnboundedReceiver, UnboundedSender, unbounded_channel},
        oneshot,
    },
    task::JoinHandle,
    time,
};

struct ConnectionState {
    sender: UnboundedSender<([u8; 1024], usize)>,
    last_active: Instant,
}

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    connections: Arc<Mutex<HashMap<String, ConnectionState>>>,
    timeout: Duration,
    shutdown_rx: UnboundedReceiver<()>,
    task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>, // Track all tasks
    // file_path: &'static str,
    songs_list: Arc<Vec<String>>,
}

impl UdpServer {
    pub async fn new(
        socket: UdpSocket,
        timeout: Duration,
        songs_list: Vec<String>,
    ) -> (Self, UnboundedSender<()>) {
        let (shutdown_tx, shutdown_rx) = unbounded_channel();
        // let file_path = Path::new(file_path.as_str());
        (
            Self {
                socket: Arc::new(socket),
                connections: Arc::new(Mutex::new(HashMap::new())),
                timeout,
                shutdown_rx,
                task_handles: Arc::new(Mutex::new(Vec::new())),
                // file_path: file_path,
                songs_list: Arc::new(songs_list),
            },
            shutdown_tx,
        )
    }

    pub async fn run(&mut self) {
        // let res = self.songs_list.clone();
        let mut interval = time::interval(Duration::from_secs(10));
        let mut buf = [0u8; 1024];
        loop {
            select! {
                // Shutdown signal
                _ = self.shutdown_rx.recv() => {
                    println!("Shutdown initiated, closing connections...");
                    self.cleanup_all_connections().await;
                    return;
                }

                // Periodic cleanup
                _ = interval.tick() => {
                    // println!("timeout check!!!!");
                    self.cleanup_stale_connections().await;
                }

                // Incoming packets
                res = self.socket.recv_from(&mut buf) => {
                    let (len, addr) = match res {
                        Ok(v) => {
                            v
                        },
                        Err(e) => {
                            eprintln!("Receive error: {}", e);
                            continue;
                        }
                    };
                    self.handle_packet(addr, len, buf).await;
                }
            }
        }
    }

    async fn cleanup_all_connections(&mut self) {
        // Close all senders first
        let mut connections = self.connections.lock().await;
        for (_, state) in connections.iter_mut() {
            drop(state.sender.clone()); // Close channel to signal tasks to exit
        }
        connections.clear();

        // Wait for all tasks to complete
        let mut handles = self.task_handles.lock().await;
        while let Some(handle) = handles.pop() {
            if let Err(e) = handle.await {
                eprintln!("Task failed during shutdown: {}", e);
            }
        }
        println!("All connections closed");
    }

    async fn cleanup_stale_connections(&mut self) {
        let now = Instant::now();
        let mut connections = self.connections.lock().await;

        connections.retain(|addr, state| {
            if now.duration_since(state.last_active) > self.timeout {
                println!("Connection from {} timed out", addr);
                false
            } else {
                true
            }
        });
    }

    async fn handle_packet(&mut self, addr: SocketAddr, len: usize, buf: [u8; 1024]) {
        let addr_str = addr.to_string();
        let mut connections = self.connections.lock().await;
        // let mut buf = [0u8; 1024];
        match connections.get_mut(&addr_str) {
            Some(state) => {
                // println!("request from client: {:?}", &buf[..len]);
                state.last_active = Instant::now();
                if len == 0 {
                    connections.remove(&addr_str);
                    println!("Connection from {} closed", addr);
                } else if let x = std::str::from_utf8(&buf[..len]) {
                    // in case the user want to quit!!!!
                    match x {
                        Ok(str) => {
                            if str == "quit" {
                                println!("Quit request called!!! Connection from {} closed", addr);
                                connections.remove(&addr_str);
                                println!("Connection from {} closed", addr);
                            } else {
                                state.sender.send((buf, len)).unwrap();
                            }
                        }
                        Err(e) => {
                            state.sender.send((buf, len)).unwrap();
                        }
                    }
                } else {
                    state.sender.send((buf, len)).unwrap();
                }
            }
            None => {
                let (tx, rx) = unbounded_channel();
                let socket = self.socket.clone();
                let timeout = self.timeout;

                let task_handle = tokio::spawn(Self::processing_task(
                    socket,
                    addr,
                    rx,
                    timeout,
                    self.songs_list.clone(),
                ));

                // Store the task handle
                self.task_handles.lock().await.push(task_handle);
                tx.send((buf, len)).unwrap();
                connections.insert(
                    addr_str.clone(),
                    ConnectionState {
                        sender: tx,
                        last_active: Instant::now(),
                    },
                );
            }
        }
    }

    async fn processing_task(
        socket: Arc<UdpSocket>,
        addr: SocketAddr,
        mut rx: UnboundedReceiver<([u8; 1024], usize)>,
        timeout: Duration,
        // list_of_song:
        songs_list: Arc<Vec<String>>,
    ) {
        println!("New connection from {}", addr);
        //load the list first.....
        // println!("{:?}", songs_list.len());
        // for i in songs_list.as_ref() {
        //     socket.send_to(i.as_bytes(), addr).await;
        //     // socket.recv_buf(buf)
        // }
        // socket.send_to("done!!!!".as_bytes(), addr);
        // //

        let socket_clone = socket.clone();
        let songs_list_clone = songs_list.clone();
        let (mut song_tx, mut song_rx) = mpsc::channel(100);
        tokio::spawn(song_processing::song_processing(
            socket_clone,
            song_rx,
            songs_list_clone,
            addr,
        ));
        loop {
            select! {
                Some((buf, len)) = rx.recv() => {
                    //quit request!!!
                    // if let xstd::str::from_utf8(&buf[..len]) == "quit"{
                    //     println!("Quit requested at first!!!!: {}", addr);
                    //     break;
                    // }
                    // entry request!!!!
                    if len==0{
                        println!("{:?}",songs_list.len());
                    for i in songs_list.as_ref(){
                        socket.send_to(i.as_bytes(), addr).await;
                        // socket.recv_buf(buf)
                    }
                        socket.send_to("".as_bytes(), addr).await;

                    }
                    else{
                        song_tx.send((buf,len)).await;

                        // println!("something is fixing");
                        // the cause of overhead!!!!

                        // need for the independence!!!!
                        // Self::song_processing(socket.clone(), buf,len, songs_list.clone(),addr).await;
                    }

                }

                // _ = time::sleep(timeout) => {
                //     println!("Connection to {} timed out", addr);
                //     return;
                // }
                else=>{
                    // println!("Done: {}", addr);
                    // drop(song_tx);

                    break;
                }
            }
            // drop(song_tx);
        }
        println!("Done: {}", addr);
    }
}
```

- the preparation
    
    ```rust
    pub struct UdpServer {
        socket: Arc<UdpSocket>,
        connections: Arc<Mutex<HashMap<String, ConnectionState>>>,
        timeout: Duration,
        shutdown_rx: UnboundedReceiver<()>,
        task_handles: Arc<Mutex<Vec<JoinHandle<()>>>>, // Track all tasks
        // file_path: &'static str,
        songs_list: Arc<Vec<String>>,
    }
    
    ```
    

Do dùng UDP nên gần như phải làm chay tất cả, từ việc xây dựng kết nối, điều phối kết nối, lo timeout,… Mọi thứ phải làm chay từ đầu đến cuối, nên phần này có thể nói rằng tốn nhiều thời gian nhất. 

- the entrance
    
    ```rust
      pub async fn run(&mut self) {
            // let res = self.songs_list.clone();
            let mut interval = time::interval(Duration::from_secs(10));
            let mut buf = [0u8; 1024];
            loop {
                select! {
                    // Shutdown signal
                    _ = self.shutdown_rx.recv() => {
                        println!("Shutdown initiated, closing connections...");
                        self.cleanup_all_connections().await;
                        return;
                    }
    
                    // Periodic cleanup
                    _ = interval.tick() => {
                        // println!("timeout check!!!!");
                        self.cleanup_stale_connections().await;
                    }
    
                    // Incoming packets
                    res = self.socket.recv_from(&mut buf) => {
                        let (len, addr) = match res {
                            Ok(v) => {
                                v
                            },
                            Err(e) => {
                                eprintln!("Receive error: {}", e);
                                continue;
                            }
                        };
                        self.handle_packet(addr, len, buf).await;
                    }
                }
            }
        }
    ```
    

Khi bắt đầu chạy, server phải thủ sẵn một đoạn buf khoảng 1KB để nhận thông tin từ phía client và từ đó đưa vào một vòng lặp. Ở đây, tôi có 3 việc lớn, đầu tiên là ngắt chương trình nếu như có tín hiệu ngắt, việc tiếp theo là cứ mỗi một chu kì nhất định, tôi sẽ kiểm tra xem các kết nối nào có dấu hiệu AFK rồi loại bỏ để giải phóng slot cho các kết nối khác, và cuối cùng, tôi xử lý một request nào đó. Trên thực tế, việc thiết lập này gần như tốt nhất tôi có thể nghĩ ra, tuy nhiên, việc kiểm tra theo chu kì này sẽ khiến cho toàn bộ chương trình bị đứng. Đáng ra, điều ổn nhất tôi nên làm là timeout trên mỗi kết nối, điều mà sau này QUIC đã thay tôi làm hết. Dẫu vậy, đó là giải pháp timeout tốt nhất mà tôi đã có thể đưa ra cho riêng mình.  Phần đấy còn chưa đủ ức chế, phải sang phần xử lý gói tin, tôi mới thấy mức độ bẩn mắt và khó chịu từ cái timeout đầy căng thẳng đấy :))))

- packet handling
    
    ```rust
     async fn handle_packet(&mut self, addr: SocketAddr, len: usize, buf: [u8; 1024]) {
            let addr_str = addr.to_string();
            let mut connections = self.connections.lock().await;
            // let mut buf = [0u8; 1024];
            match connections.get_mut(&addr_str) {
                Some(state) => {
                    // println!("request from client: {:?}", &buf[..len]);
                    state.last_active = Instant::now();
                    if len == 0 {
                        connections.remove(&addr_str);
                        println!("Connection from {} closed", addr);
                    } else if let x = std::str::from_utf8(&buf[..len]) {
                        // in case the user want to quit!!!!
                        match x {
                            Ok(str) => {
                                if str == "quit" {
                                    println!("Quit request called!!! Connection from {} closed", addr);
                                    connections.remove(&addr_str);
                                    println!("Connection from {} closed", addr);
                                } else {
                                    state.sender.send((buf, len)).unwrap();
                                }
                            }
                            Err(e) => {
                                state.sender.send((buf, len)).unwrap();
                            }
                        }
                    } else {
                        state.sender.send((buf, len)).unwrap();
                    }
                }
                None => {
                    let (tx, rx) = unbounded_channel();
                    let socket = self.socket.clone();
                    let timeout = self.timeout;
    
                    let task_handle = tokio::spawn(Self::processing_task(
                        socket,
                        addr,
                        rx,
                        timeout,
                        self.songs_list.clone(),
                    ));
    
                    // Store the task handle
                    self.task_handles.lock().await.push(task_handle);
                    tx.send((buf, len)).unwrap();
                    connections.insert(
                        addr_str.clone(),
                        ConnectionState {
                            sender: tx,
                            last_active: Instant::now(),
                        },
                    );
                }
            }
        }
    
        async fn processing_task(
            socket: Arc<UdpSocket>,
            addr: SocketAddr,
            mut rx: UnboundedReceiver<([u8; 1024], usize)>,
            timeout: Duration,
            // list_of_song:
            songs_list: Arc<Vec<String>>,
        ) {
            println!("New connection from {}", addr);
            //load the list first.....
            // println!("{:?}", songs_list.len());
            // for i in songs_list.as_ref() {
            //     socket.send_to(i.as_bytes(), addr).await;
            //     // socket.recv_buf(buf)
            // }
            // socket.send_to("done!!!!".as_bytes(), addr);
            // //
    
            let socket_clone = socket.clone();
            let songs_list_clone = songs_list.clone();
            let (mut song_tx, mut song_rx) = mpsc::channel(100);
            tokio::spawn(song_processing::song_processing(
                socket_clone,
                song_rx,
                songs_list_clone,
                addr,
            ));
            loop {
                select! {
                    Some((buf, len)) = rx.recv() => {
                        //quit request!!!
                        // if let xstd::str::from_utf8(&buf[..len]) == "quit"{
                        //     println!("Quit requested at first!!!!: {}", addr);
                        //     break;
                        // }
                        // entry request!!!!
                        if len==0{
                            println!("{:?}",songs_list.len());
                        for i in songs_list.as_ref(){
                            socket.send_to(i.as_bytes(), addr).await;
                            // socket.recv_buf(buf)
                        }
                            socket.send_to("".as_bytes(), addr).await;
    
                        }
                        else{
                            song_tx.send((buf,len)).await;
    
                            // println!("something is fixing");
                            // the cause of overhead!!!!
    
                            // need for the independence!!!!
                            // Self::song_processing(socket.clone(), buf,len, songs_list.clone(),addr).await;
                        }
    
                    }
    
                    // _ = time::sleep(timeout) => {
                    //     println!("Connection to {} timed out", addr);
                    //     return;
                    // }
                    else=>{
                        // println!("Done: {}", addr);
                        // drop(song_tx);
    
                        break;
                    }
                }
                // drop(song_tx);
            }
            println!("Done: {}", addr);
        }
    }
    ```
    

Theo như phần packet handling, thì khi mới vào, tại hàm get_mut() lấy thông tin client dựa trên địa chỉ IP và port, trả về một mutable reference của chính client đó, bao gồm thông tin, timeout, quá trình xử lý…. Nếu có địa chỉ dựa trên khóa đó, thì xử lý thông tin của gói tin đấy, bao gồm yêu cầu lấy thông tin nhạc và thoát chương trình. Trong trường hợp không có thông tin, tức là client chỉ mới vào lần đầu, hoặc đã bị timeout, thì server lại ghi vào trong sổ, bằng hash map. Việc tạo ra kết nối mới cũng tiến hành luôn quá trình xử lý gói tin và tạo ra một channel để tiện cho việc tách phần xử lý gói tin và gửi thông tin nhạc, tránh các phức tạp không cần thiết. Tuy nhiên, như tôi nói, phần này trở nên quá dài, do viết bằng UDP thuần, nên không có một cơ chế xử lý timeout triệt để, cũng như sắp xếp các yếu tố này hơi tệ, dẫn đến một bug của chương trình là nếu cho hàm sleep vào quá trình phát nhạc thì nó sẽ treo vĩnh viễn dù cố dùng Ctrl+C để ngắt mà sau này tôi mới tìm được lỗi khi nhìn vào phần spawning của quá trình phát nhạc, là rõ ràng không có một cơ chế báo hiệu. Có thể nói là vừa vất vả, lãng phí nhưng cực kì bẩn mắt.

Dẫu vậy, đó cũng là cố gắng hết sức của tôi trong vòng 1-2 tuần để làm xong, mặc dù chương trình chạy không thực sự tốt như tôi nghĩ, nhưng mà nó là cả sự cố gắng mà tôi đã đưa ra. Ít nhất là những gánh nặng này thực sự kết thúc khi tôi làm đến phần QUIC.

## part 3: first time hearing sound

### complete version in UDP packet loss prevention, both in server and client

- in server
    
    ```rust
    use std::{net::SocketAddr, process::Output, sync::Arc, time::Duration};
    
    use rodio::Decoder;
    use tokio::{
        net::UdpSocket,
        select,
        sync::mpsc::{self, Receiver, UnboundedReceiver, UnboundedSender, unbounded_channel},
        time::sleep,
    };
    // use tokio_util::sync::CancellationToken;
    
    pub async fn song_processing(
        socket: Arc<UdpSocket>,
        mut song_rx: Receiver<([u8; 1024], usize)>,
        songs_list: Arc<Vec<String>>,
        addr: SocketAddr,
    ) {
        let (control, mut song_id_rx, mut seq_rx, _) = control(song_rx);
        let (mut chunk_tx, mut chunk_rx) = unbounded_channel();
        // seq_rx.recv_many(buffer, limit)
        tokio::spawn(song_finding(song_id_rx, songs_list, chunk_tx));
        tokio::spawn(control);
    
        // let mut id_check = 0usize;
        // let mut seq_check = 0u8;
        //for caching, if packet loss
        let mut cache: Vec<u8> = Vec::with_capacity(260);
        let mut user_req: u8 = 0; // -> request chunk
        let mut present: u8 = 0; // -> newest chunk possible
    
        /*
        packet loss occur when user_req != present (user_req != present)
    
        if user_req == present -> user request the newest chunk possible, increase the present, save the cache for later use
        if user_req != present -> packet loss occurred -> request the cache
         */
        loop {
            // let seq = seq_rx.recv().await;
            match seq_rx.recv().await {
                Some(index) => {
                    user_req = index;
                    // println!("user_req: {}", user_req);
                }
                None => {
                    break;
                }
            }
            // let mut received_chunk: Vec<u8> = Vec::with_capacity(260);
            if user_req == present {
                //request the newest chunk
                // println!("requesting newest chunk!!");
                match chunk_rx.recv().await {
                    Some(chunk) => {
                        // let check = chunk.clone();
                        // if &check == "done".as_bytes() {
                        //     socket.send_to("done!!!".as_bytes(), addr).await;
                        // } else {
                            cache = chunk;
                        // }
                    }
                    None => {
                        // socket.send_to("done!!!".as_bytes(), addr).await;
                        // println!("done request sent!!!!");
                        break;
                    }
                }
                //update the cache, also update the cache
                //send the cache
                // socket.send_to(&cache, addr).await;
                // let test_buf = [user_req];
                // socket.send_to(&test_buf, addr).await;
                // println!("{}",cache.len());
                socket.send_to(&cache, addr).await;
                // println!("sending {}", cache[cache.len() - 1]);
                //update the present
                present = add(present);
            } else {
                // println!("requesting cache chunk!!");
                // let test_buf = [user_req];
                // println!("{}",cache.len());
                socket.send_to(&cache, addr).await;
                // println!("sending {}", cache[cache.len() - 1]);
                //only send the cache -> packet loss detected!!!
            }
            // last byte of chunk == present -> send
        }
        // println!("nothing to see");
        // let (mut song_tx, mut song_rx) = mpsc::unbounded_channel();
    
        // println!("")
        /*
        plan:
        if a song -> spawn the song tasks, don't wait...
    
        if a chunk ->
         */
    
        // loop {}
    }
    fn adding(i: u8) -> u8 {
        if i == 255 {
            return 0;
        } else {
            return i + 1;
        }
    }
    pub async fn song_finding(
        mut find_song_rx: UnboundedReceiver<usize>,
        songs_list: Arc<Vec<String>>,
        chunk_tx: UnboundedSender<Vec<u8>>,
    ) {
        while let Some(index) = find_song_rx.recv().await {
            println!("song index called!!! {}", index);
            if index >= songs_list.len() {
                // socket.send_to("".as_bytes(), addr).await;
                chunk_tx.send(vec![]);
                continue;
            }
            let song_name = &songs_list[index];
            let file = std::io::BufReader::new(std::fs::File::open(song_name.as_str()).unwrap());
            let mut source = Decoder::new(file).unwrap();
    
            // source.
            let buffer: Vec<i16> = source.by_ref().collect();
            println!("len of song: {}",buffer.len());
            let bytes: Vec<u8> = buffer
                .iter()
                .flat_map(|&num| num.to_be_bytes().to_vec()) // little-endian
                // .flat_map(|&num| num.to_be_bytes().to_vec()) // big-endian
                .collect();
            let mut iter = bytes.chunks(1024); // 1mb per chunk....
            let mut seq = 0;
            while let Some(res) = iter.next() {
                // let mut salt = "music_chunk".as_bytes().to_vec();
                let mut res = res.to_owned();
                res.push(seq);
                // res.append(&mut salt);
                chunk_tx.send(res);
                seq = add(seq); // i = i+1;
            }
            // let mut done = "done".as_bytes().to_vec();
            // done.push(seq);
            // chunk_tx.send(done);
            // drop(tx);
        }
        drop(chunk_tx);
    }
    fn add(mut i: u8) -> u8 {
        if i == 255 {
            return 0;
        } else {
            return i + 1;
        }
    }
    
    pub fn control(
        mut song_rx: Receiver<([u8; 1024], usize)>,
    ) -> (
        impl Future<Output = ()>,
        UnboundedReceiver<usize>,
        UnboundedReceiver<u8>,
        UnboundedReceiver<()>,
    ) {
        let (mut find_song_tx, mut find_song_rx) = mpsc::unbounded_channel(); //find song command
        let (mut seq_tx, mut seq_rx) = mpsc::unbounded_channel(); //sequence number
        let (mut fail_alert_tx, mut fail_alert_rx) = mpsc::unbounded_channel(); //wrong command
    
        // let (mut find_song_tx, mut find_song_rx) = mpsc::unbounded_channel(); let token = CancellationToken::new();
        // let token = CancellationToken::new();
        // let cancellation = token.clone();
        //1kb per chunk.
        let operate = async move {
            while let Some(res) = song_rx.recv().await {
                let buf = res.0;
                let len = res.1;
                // println!("what's received!!!: {:?}", &buf[..len]);
                //check if the sequence packet
                let seq_check = std::str::from_utf8(&buf[..3]).unwrap();
                if seq_check == "seq" {
                    // println!("sequence packet detected!!!!");
                    // println!("{:?}", &buf[]);
                    let seq_num = buf[len - 1];
                    // println!("{}", seq_num);
                    seq_tx.send(seq_num);
                    continue;
                }
                match std::str::from_utf8(&buf[..len]) {
                    Err(_) => {
                        fail_alert_tx.send(());
                    }
                    Ok(res) => match res.parse::<usize>() {
                        Ok(res) => {
                            find_song_tx.send(res);
                        }
                        Err(_) => {
                            fail_alert_tx.send(());
                        }
                    },
                }
            }
            drop(find_song_tx);
            drop(seq_tx);
            drop(fail_alert_tx);
            // println!("control task is done!!!!");
        };
        (operate, find_song_rx, seq_rx, fail_alert_rx)
    }
    ```
    
- in client
    
    ```rust
    fn network_block(
        // socket: Arc<UdpSocket>,
        // addr: SocketAddr,
        mut song_request_receiver: Receiver<Vec<u8>>,
        // cancellation: CancellationToken,
        client_socket: Arc<UdpSocket>,
    ) -> (UnboundedReceiver<Vec<u8>>, impl Future<Output = ()>) {
        // cancellation.cancelled()
        let (chunk_tx, mut chunk_rx) = unbounded_channel();
        let network_loop = async move {
            while let Some(res) = song_request_receiver.recv().await {
                println!("received");
                client_socket.send(&res).await;
                let mut seq = [115, 101, 113, 0];
                loop {
                    let mut buf = Vec::with_capacity(1025);
                    client_socket.send(&seq).await;
                    if let Err(_) = tokio::time::timeout(
                        tokio::time::Duration::from_secs(60),
                        client_socket.recv_buf(&mut buf),
                    )
                    .await
                    {
                        // println!("out of time!!!!");
                        // drop(sender);
                        //reset
                        break;
                    }
                    // println!("{}", buf.len());
                    // size += (buf.len() - 1);
                    chunk_tx.send(buf[..buf.len() - 1].to_vec());
                    // let res = rodio::buffer::SamplesBuffer::new(2, 48000, res);
                    // s1.lock().unwrap().append(res);
                    //         tx.send(res).await;
                    if buf[buf.len() - 1] == seq[3] {
                        // println!("received!!! {}", seq[3]);
                        seq[3] = adding(seq[3]);
                    }
                }
            }
            println!("control block done, dropping audio block");
            drop(chunk_tx);
        };
        (chunk_rx, network_loop)
    }
    
    ```
    

Sang phần 3, tôi muốn nói về vấn đề nghiêm trọng nhất và mang tính nền tảng, đó là hiện tượng packet loss, hay còn gọi là “mất gói tin”. Bản chất, nếu như không có một cơ chế kiểm soát đường truyền, thì rất có thể hiện tượng mất gói tin sẽ xuất hiện vì UDP không có cơ chế để bảo toàn tính toàn vẹn dữ liệu như TCP, mà TCP một khi bị mất dữ liệu sẽ dừng hẳn cả đường truyền chỉ để cố gửi tại gói tin bị mất. Biểu hiện của việc bị mất gói tin chính là hiện tượng xước đĩa, cũng như thời lượng bị rút ngắn hơn bình thường. Vì lẽ đó, tôi phải có một cơ chế đủ tốt để kiểm tra tính toàn vẹn của gói tin, cũng như kiểm soát luồng. Thì, ở phần client cũng như server, tôi đã thêm vào 1 byte để đánh dấu xem là tiến trình đọc file tới đâu, cũng như kiểm tra xem có đúng như yêu cầu không, cái cơ chế này gần giống với TCP. Nhờ việc kiểm tra liên tục và có trật tự, gần như chương trình được kiểm soát tốt và không còn tình trạng bị mất gói tin. Tuy nhiên, kết cục thì vẫn còn đó, làm như vậy cũng đã khiến chương trình phức tạp đi rất nhiều mà xa rời bản chất, là phát nhạc. Mục tiêu của phần mềm là phát nhạc đơn giản chứ không nên bị phức tạp hóa, nhưng hầu hết quá trình làm đều dành cho việc điều khiển và kiểm soát nhạc, vì thế, thời gian bị ăn bớt nhiều mà không có một giải pháp thực sự tốt.

## part 4: QUIC - finest control without compromises - the pinnacle.

Riêng phần này, là phần kết thúc cho toàn bộ quá trình, nhưng điểm đặc biệt là tôi quyết định không viết thêm chương trình mà chỉ viết một đoạn đơn giản. Trên thực tế, mục tiêu chương trình rõ ràng là demo khả năng làm việc của Rust nhiều hơn là cố gắng viết một chương trình quá phức tạp nhưng kém hiệu quả. Quay trở lại tư duy ban đầu, tôi đơn giản hóa chương trình chỉ là phát nhạc đơn thuần. Và, trong vòng 2 tuần cuối tháng, tôi nghiên cứu về QUIC cũng như cách áp dụng. Thật tuyệt khi QUIC lại có cơ chế kiểm soát lưu lượng và điều khiển khá tốt mà tốc độ của UDP cũng không đánh đổi nhiều. Và đúng như tựa đề, khả năng kiểm soát cao nhất mà không phải đánh đổi, đỉnh cao của dự án.

code section

- client
    
    ```rust
    mod client_util;
    use std::time::Duration;
    
    use client_util::{run_unsafe_client, setup_unsafe};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        select,
        sync::mpsc::unbounded_channel,
    };
    use tokio_util::sync::CancellationToken;
    #[tokio::main]
    async fn main() {
        //setup connection
        use rustls::crypto::ring::default_provider;
        default_provider().install_default().unwrap();
        
        let addr = std::net::SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(192, 168, 0, 102)),
            8080,
        );
        let (mut connection, mut endpoint) = setup_unsafe(addr).await.unwrap();
        let (mut send_stream, mut recv_stream) = connection.accept_bi().await.unwrap();
        // sound....
        let (stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
        let sink = std::sync::Arc::new(std::sync::Mutex::new(
            rodio::Sink::try_new(&stream_handle).unwrap(),
        ));
        let cancellation = CancellationToken::new();
        let c = cancellation.clone();
        let mut s1 = sink.clone();
        let (sender, mut receiver) = unbounded_channel();
        let t1 = tokio::spawn(async move {
            loop {
                let mut buf = [0u8; 1000];
                let res = recv_stream.read_exact(&mut buf).await;
                match res {
                    Ok(x) => {
                        // println!("{}: with {}", recv_stream.id(), n);
                        let res: Vec<i16> = buf
                            .chunks_exact(2)
                            .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                            .collect();
                        let res = rodio::buffer::SamplesBuffer::new(2, 40000, res);
                        // s1.lock().unwrap().append(res);
                        sender.send(res);
                        // let wait_buffer = [0u8; 1];
                        
                        // send_stream.write_all(&wait_buffer).await;
                        // tokio::time::sleep(Duration::from_millis(100)).await;
                        // send_stream.flush().await;
                    }
    
                    Err(e) => {
                        println!("last piece of the song!!!!");
                        let res: Vec<i16> = buf
                            .chunks_exact(2)
                            .map(|chunk| i16::from_be_bytes(chunk.try_into().unwrap()))
                            .collect();
                        let res = rodio::buffer::SamplesBuffer::new(2, 40000, res);
                        // s1.lock().unwrap().append(res);
                        sender.send(res);
                        break;
                    }
                }
                
            }
            cancellation.cancel();
        });
        let t3 = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(500)).await;
            while let Some(res) = receiver.recv().await {
                s1.lock().unwrap().append(res);
            }
        });
        let t2 = tokio::spawn(async move {
            select! {
                _ = c.cancelled()=>{
                    sink.lock().unwrap().sleep_until_end();
                }
            }
        });
        tokio::try_join!(t1, t2, t3).unwrap();
        // t1.await;
        // sink.lock().unwrap().sleep_until_end();
        // select! {
        //     _ = t1=>{
        //         sink.lock().unwrap().sleep_until_end();
        //     }
        // };
    }
    ```
    
- server
    
    ```rust
    use std::{
        io::Read,
        net::{SocketAddr, SocketAddrV4}, time::Duration,
    };
    
    use rustls::crypto::aws_lc_rs::default_provider;
    use tokio::{io::AsyncWriteExt, time::sleep};
    
    mod server_util;
    #[tokio::main]
    async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
        use rustls::crypto::ring::default_provider;
        default_provider().install_default().unwrap();
        //decompress the sound chunk
        let file = std::io::BufReader::new(
            std::fs::File::open("./list_songs/ThatllBeTheDay-ShawnMendes-16714907_hq.mp3").unwrap(),
        );
        // std::io::pipe()
        let mut source = rodio::Decoder::new(file).unwrap();
    
        let buffer: Vec<i16> = source.by_ref().collect();
        let bytes: Vec<u8> = buffer
            .iter()
            .flat_map(|&num| num.to_be_bytes().to_vec())
            .collect();
    
        //create server endpoint...
        let addr = SocketAddr::new(
            std::net::IpAddr::V4(std::net::Ipv4Addr::new(0, 0, 0, 0)),
            8080,
        );
    
        let (endpoint, _) = server_util::make_server_endpoint(addr).unwrap();
        let incoming = endpoint.accept().await.unwrap();
        let mut conn = incoming.await.unwrap();
        let mut bi = conn.open_bi().await.unwrap();
        // uni.
        // implement the ack!!!!
        let mut iter = bytes.chunks(4410);
        while let Some(chunk) = iter.next(){
    
            bi.0.write_all(chunk).await;
            
            sleep(Duration::from_millis(10)).await;
            bi.0.flush().await;
            println!("sended!!!");
            // sleep(Duration::from_millis(10)).await;
            // let mut ack_buffer = [0u8;1];
            // bi.1.read_exact(&mut ack_buffer).await;
            // println!("ack received!!!");
    
        }
        // while let Some(chunk) = iter.next() {
        //     // uni.stopped()
        //     uni.write(chunk).await;
        //     sleep(Duration::from_millis(1)).await;
        //     uni.flush();
        // }
        
        bi.0.finish();
        bi.0.shutdown().await;
        // uni.stopped().await;
        //send the sound to the client...
        Ok(())
    }
    ```
    

Tôi lấy ngay dàn ý của phần experimental và biến tấu lại. Sau khi đọc file xong, server sẽ cắt toàn bộ file nhạc thành các đoạn chunk và ngay khi có tín hiệu từ client, server sẽ vào việc. Cái hay của QUIC là cho phép streaming không khác gì TCP với connection_id được định sẵn. Điều hay nhất là QUIC cũng đã thiết lập cơ chế timeout cho mỗi connection, đảm bảo về khả năng độc lập cho mỗi kết nối mà không ảnh hưởng đến toàn bộ chương trình. Nhờ việc đó, mà chương trình được rút ngắn đi rất nhiều và nếu sau này, có khi tôi ưu tiên QUIC vì độ trễ thấp, tốc độ cao, khả năng kiểm soát tuyệt đối và bảo mật hơn TCP. 

Và thế là toàn bộ dự án kết thúc trong êm đẹp và tôi có hẳn 1 tháng để viết về phần giải thích này.

# iv. what i have learned and impacts.

so, the last song in this tour….

“it isn’t in my blood”

[https://open.spotify.com/track/2QZ7WLBE8h2y1Y5Fb8RYbH?si=5b74efc59f084d7e](https://open.spotify.com/track/2QZ7WLBE8h2y1Y5Fb8RYbH?si=5b74efc59f084d7e)

Thực ra thì, phải sau hẳn 1 tháng làm xong dự án và trải qua việc bị cạn ý tưởng, tôi cũng dành ra được thời gian để viết tiếp. Để bắt đầu cho một dự án như này, nếu lớn thì cũng không hẳn, nhưng đã đủ căng não cho một dự án rồi. Giống như lời hứa ban đầu khi tôi quyết định làm dự án, là việc tôi phải làm từ không có gì, cho đến việc hoàn thành một chương trình ở cấp độ cao nhất và từ dự án này, nó quyết định cả cái cách tôi suy nghĩ về sau, cũng như đánh dấu cho việc tôi bước ra khỏi 6 năm và dần hòa nhập vào hiện tại.

Một cách trực tiếp nhất, tôi đã làm được với Rust. Không phải ngẫu nhiên mà tôi nói vậy đâu. Cái khó nhất cho tôi khi học Rust là làm thế nào để làm việc với một tư duy bộ nhớ khắc nghiệt bậc nhất, cũng như việc tôi không có một background nhất định nào. Tất cả, tôi đều tự nghiên cứu từ đầu tới cuối nên có thể nói rằng, tôi vui vì mình đã không bỏ cuộc và đã đi đến cuối cùng. Đặc biệt nhất, tôi cũng đã hiểu được asynchronous và concurrency qua chính dự án này, những bản mẫu, thiết kế và lý luận đằng sau. Sự háo hức từ việc học được điều gì đúng đắn sau nhiều năm đưa tôi từ niềm vui này sang niềm vui khác, dẫn đến tiến độ được đẩy nhanh hơn đáng kể.

Tiếp đến, tôi thích việc nghiên cứu và coi nó là một kĩ năng nhiều hơn chỉ là cho một nghề nghiệp. Hầu hết những gì tôi nghiên cứu được và tìm hiểu trong vòng 10 năm đổ lại, đều có trên phần cơ sở lý luận. Tuy vậy, để bắt đầu lý luận, chúng ta phải học được cách tháo bỏ những định kiến và thử đưa ra nhiều nghi vấn nhấ có thể, điều này sẽ giúp cho chúng ta định hình được phương pháp luận. Hồi bé, tôi là một thằng tìm tòi nhiều về điện tử, đặc biệt là TV trước khi tôi học lập trình. Đến giờ, tôi vẫn thích những quảng cáo của Sony, đặc biệt là trong năm 2017-2019 với các dòng TV OLED và LCD cao cấp. Những hiểu biết và lý luận này sẽ đi theo tôi đến cả cuộc đời mà tôi không nghĩ rằng, đôi lúc tôi sẽ rất cần những dự án đặc biệt như thế này.

Và cuối cùng, đã làm một dự án, chuyện thất bại là bình thường, hay đúng hơn là, chúng ta nên cảm thấy tốt hơn khi học được nhiều từ thất bại, chứ không phải cảm thấy thất bại. Lúc tôi bắt đầu dự án thì mẹ tôi thúc giục và hỏi chuyện tôi đi thực tập lúc nào. Cái việc đó, đúng, mẹ tôi lo là đúng, nhưng thực sự tôi đã gồng ép bản thân mình quá nhiều vì lo sợ sẽ bị đánh trượt CV. Cái nỗi sợ này đeo bám tôi trong nhiều năm khi tôi mất đi hoàn toàn ý chí sống. Sau này, tôi mới hiểu ra rằng sẽ không thực sự có cái gì là ổn định cả, hay đúng hơn là, thứ ở lại với chúng ta ngoài trải nghiệm, kinh nghiệm và kiến thức, còn lại thì cũng đến và đi, nhưng không có nghĩa là ta phải cảm thấy buồn vì điều đó. Học đối diện với thất bại một cách nhẹ nhàng, tôi trở nên vui vẻ hơn và biết rằng, kể cả khi dự án này thất bại và không thể đi xa hơn, tôi cũng đã đi thực sự rất xa rồi, nó mới chỉ là bắt đầu cho một cái gì đó thực sự mới. Điều hay nhất là, tôi không bỏ cuộc và tôi không hối hận vì đã cố gắng, nhờ nó mà tôi hiểu được nhiều mảng kiến thức bị bỏ quên, giống như cách tôi học Pascal từ C ấy, có lúc nào nó dễ dàng đâu. Nói trắng ra thì, đây chính là quà sinh nhật cho tôi khi có thể bước ra khỏi một cái gì đó kìm hãm và tìm tới niềm vui trong mọi công việc mình làm và không thấy gượng ép.

Dù sao thì, phần này tôi không viết dài, tôi cũng không nói về code nhiều mà tôi muốn tập trung vào cơ sở lý luận của dự án - thứ mà tôi đã bỏ quên khi chỉ tập trung nhìn vào code của người khác và luôn nghĩ là nó phải chạy được mà không có một lý do thỏa đáng. Và, từ giờ nó đánh dấu cho một giai đoạn lập trình mới và vui vẻ hơn sau này.

anyway, im happy with my own.

### v. references.

- Chinese guzheng: [http://english.cssn.cn/skw_culture/202402/t20240201_5731908.shtml](http://english.cssn.cn/skw_culture/202402/t20240201_5731908.shtml)
- Music notation: [https://www.britannica.com/art/musical-notation](https://www.britannica.com/art/musical-notation)
- First telephone: [https://heritagecalling.com/2022/07/29/the-story-behind-the-worlds-first-telephone/](https://heritagecalling.com/2022/07/29/the-story-behind-the-worlds-first-telephone/)
- How was the telephone invented?: [https://www.sciencemuseum.org.uk/objects-and-stories/ahoy-alexander-graham-bell-and-first-telephone-call](https://www.sciencemuseum.org.uk/objects-and-stories/ahoy-alexander-graham-bell-and-first-telephone-call)
- Bravia A1E: [https://www.rtings.com/tv/reviews/sony/a1e-oled](https://www.rtings.com/tv/reviews/sony/a1e-oled)
- Microphone: [https://www.audio-technica.com/en-us/support/a-brief-guide-to-microphones-what-a-microphone-does](https://www.audio-technica.com/en-us/support/a-brief-guide-to-microphones-what-a-microphone-does)
- How telephone works: [https://www.britannica.com/video/Overview-invention-telephone-focus-work-Alexander-Graham/-192158](https://www.britannica.com/video/Overview-invention-telephone-focus-work-Alexander-Graham/-192158)
- Shawn album vinyl disc: [https://shop.shawnmendesofficial.com/collections/vinyl](https://shop.shawnmendesofficial.com/collections/vinyl)
- Sony turntable: [https://electronics.sony.com/audio/audio-components/turntables/p/pslx310bt](https://electronics.sony.com/audio/audio-components/turntables/p/pslx310bt)
