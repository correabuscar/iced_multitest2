use iced::{Application, Element, Renderer, Settings, Theme};
use iced::application::StyleSheet;
use iced_native::Command;
use iced_native::subscription::{self, Subscription};
use iced_native::futures::channel::mpsc;
use iced_native::widget::text;

#[derive(Debug, Clone)]
pub enum Event {
    Ready(mpsc::Sender<Input>),
    WorkFinished,
    // ...
}

pub enum Input {
    DoSomeWork,
    // ...
}

enum State {
    Starting,
    Ready(mpsc::Receiver<Input>),
}

struct Ui {
    sender: Option<mpsc::Sender<Input>>,

}

#[derive(Debug, Clone)]
enum Message {
    ExternalMessageReceived(Event),
}


impl Application for Ui {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        let app = Ui {

            sender: None,
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("wtw")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::ExternalMessageReceived(e) => {
                println!("{:?}",e);
                match e {
                    Event::Ready(sender) => {
                        println!("set sender {:?}",sender);
                        self.sender=Some(sender);
                    }
                    Event::WorkFinished => {

                    }
                }

            }
        }
        return Command::none();
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        text("wtw").into()
    }

    fn theme(&self) -> Self::Theme {

        return Theme::Dark;
    }


    fn subscription(&self) -> Subscription<Self::Message> {

        static mut COUNT:u32=0;
        unsafe {
            COUNT += 1;
            println!("subscription called: {COUNT}");
        }
        return some_worker().map(Message::ExternalMessageReceived);
    }

    // fn scale_factor(&self) -> f64 {
    //     todo!()
    // }
    //
    // fn run(settings: Settings<Self::Flags>) -> iced::Result where Self: 'static {
    //     todo!()
    // }
}
fn main() {
    let r=Ui::run(
        Settings {

            //Settings::with_flags(UiFlags { receiver }),
            id: None,
            window: Default::default(),
            //flags: UiFlags { receiver },
            flags: (),
            default_font: None,
            default_text_size: 20.0,
            text_multithreading: false,
            antialiasing: false,
            exit_on_close_request: true,
            //..Settings::default()
            try_opengles_first: false,
        }
    );
    println!("Hello, world!");
}

fn some_worker() -> Subscription<Event> {
    struct SomeWorker;
    static mut COUNT:u32=0;
    unsafe {
        COUNT += 1;
        println!("some_worker called: {COUNT}");
    }
    subscription::unfold(std::any::TypeId::of::<SomeWorker>(),
                         State::Starting,
                         |state| async move {
                             static mut COUNT:u32=0;
                             unsafe {
                                 COUNT += 1;
                                 println!("sub::unfold called: {COUNT}");
                             }
        match state {
            State::Starting => {
                // Create channel
                let (sender, receiver) = mpsc::channel(100);

                (Some(Event::Ready(sender)), State::Ready(receiver))
            }
            State::Ready(mut receiver) => {
                use iced_native::futures::StreamExt;

                // Read next input sent from `Application`
                let input = receiver.select_next_some().await;

                match input {
                    Input::DoSomeWork => {
                        // Do some async work...

                        // Finally, we can optionally return a message to tell the
                        // `Application` the work is done
                        (Some(Event::WorkFinished), State::Ready(receiver))
                    }
                }
            }
        }
    })
}