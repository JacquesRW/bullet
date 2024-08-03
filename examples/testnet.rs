/*
This is used to confirm non-functional changes for bullet.
*/
use bullet_lib::{
    inputs, lr, optimiser, outputs, wdl, Activation, LocalSettings, Loss, TrainerBuilder, TrainingSchedule,
};

fn main() {
    let mut trainer = TrainerBuilder::default()
        .quantisations(&[181, 64])
        .optimiser(optimiser::AdamW)
        .input(inputs::Chess768)
        .output_buckets(outputs::Single)
        .feature_transformer(32)
        .activate(Activation::SCReLU)
        .add_layer(1)
        .build();

    trainer.load_from_checkpoint("checkpoints/testnet");

    let schedule = TrainingSchedule {
        net_id: "testnet".to_string(),
        eval_scale: 400.0,
        ft_regularisation: 0.0,
        batch_size: 16_384,
        batches_per_superbatch: 1,
        start_superbatch: 1,
        end_superbatch: 5,
        wdl_scheduler: wdl::ConstantWDL { value: 0.2 },
        lr_scheduler: lr::ConstantLR { value: 0.001 },
        loss_function: Loss::SigmoidMSE,
        save_rate: 10,
        optimiser_settings: optimiser::AdamWParams {
            decay: 0.01,
            beta1: 0.9,
            beta2: 0.999,
            min_weight: -1.98,
            max_weight: 1.98,
        },
    };

    let settings = LocalSettings {
        threads: 4,
        data_file_paths: vec!["../../data/batch1.data"],
        test_set: None,
        output_directory: "checkpoints",
    };

    trainer.run(&schedule, &settings);
}
