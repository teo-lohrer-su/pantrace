use crate::internal::models::TracerouteReply;

pub type TracerouteReader = dyn Iterator<Item = anyhow::Result<Vec<TracerouteReply>>>;

pub trait TracerouteWriter {
    fn write_preamble(&mut self) {
        /* do nothing by default */
    }
    fn write_epilogue(&mut self) {
        /* do nothing by default */
    }
    fn write_traceroute(&mut self, replies: &[TracerouteReply]) -> anyhow::Result<()>;
}
