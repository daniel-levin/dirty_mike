use perf_event::Builder;
use perf_event::ReadFormat;
use perf_event::SampleFlag;
use perf_event::events::Raw;

/*
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x13c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, -1, PERF_FLAG_FD_CLOEXEC) = 3
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x20010d, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, PERF_FLAG_FD_CLOEXEC) = 4
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x23c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, PERF_FLAG_FD_CLOEXEC) = 5
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x3c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, PERF_FLAG_FD_CLOEXEC) = 7
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x2c2, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, PERF_FLAG_FD_CLOEXEC) = 8
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x10e, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, PERF_FLAG_FD_CLOEXEC) = -1 EINVAL (Invalid argument)
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x10e, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, exclude_guest=1, ...}, 1080987, -1, 3, 0) = -1 EINVAL (Invalid argument)
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x10e, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, 3, 0) = -1 EINVAL (Invalid argument)
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x10e, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING|PERF_FORMAT_ID|PERF_FORMAT_GROUP, inherit=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, 3, 0) = -1 EINVAL (Invalid argument)
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x13c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 3
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x20010d, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 4
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x23c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 5
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x3c, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 7
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x2c2, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 8
perf_event_open({type=PERF_TYPE_RAW, size=0x88 /* PERF_ATTR_SIZE_??? */, config=0x10e, sample_period=0, sample_type=PERF_SAMPLE_IDENTIFIER, read_format=PERF_FORMAT_TOTAL_TIME_ENABLED|PERF_FORMAT_TOTAL_TIME_RUNNING, disabled=1, inherit=1, enable_on_exec=1, precise_ip=0 /* arbitrary skid */, ...}, 1080987, -1, -1, 0) = 9

*/
use rand::prelude::SliceRandom;

fn main() -> anyhow::Result<()> {
    fn work(data: &[i32]) -> u64 {
        let mut sum = 0u64;
        for &value in data {
            if value >= 16384 {
                sum = sum.wrapping_add(value as u64);
            }
        }
        sum
    }

    let mut shuffled_data: Vec<i32> = (0..320_768).collect();
    let mut rng = rand::rng();
    shuffled_data.shuffle(&mut rng);

    let mut sorted_data = shuffled_data.clone();
    sorted_data.sort();

    let rf = ReadFormat::TOTAL_TIME_ENABLED
        | ReadFormat::TOTAL_TIME_RUNNING
        | ReadFormat::ID
        | ReadFormat::GROUP;

    let mut b = Builder::new(Raw::new(0x13c))
        .sample(SampleFlag::IDENTIFIER)
        .read_format(
            ReadFormat::TOTAL_TIME_ENABLED
                | ReadFormat::TOTAL_TIME_RUNNING
                | ReadFormat::ID
                | ReadFormat::GROUP,
        )
        .enable_on_exec(true)
        .exclude_kernel(false)
        .exclude_hv(false)
        .exclude_guest(true)
        .inherit(true)
        .build_group()?;

    let mut handles = vec![];

    let mut setup = |code: u64| {
        let mut b1 = Builder::new(Raw::new(code));
        let b1 = b1
            .inherit(true)
            .exclude_kernel(false)
            .exclude_hv(false)
            .sample(SampleFlag::IDENTIFIER)
            .read_format(rf);

        b1.attrs_mut().set_disabled(0);

        handles.push(b.add(b1).unwrap());
    };

    setup(0x20010d);
    setup(0x23c);
    setup(0x3c);
    setup(0x2c2);
    setup(0x10e);

    b.enable()?;
    work(&shuffled_data);
    b.disable()?;
    let counts = b.read()?;

    dbg!(counts);

    Ok(())
}
