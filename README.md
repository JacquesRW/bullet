<div align="center">

# bullet

</div>

A work-in-progress Network Trainer, used to train [akimbo](https://github.com/jw1912/akimbo)'s networks.

It currently supports architectures of the form `Input -> Nx2 -> 1`, and can train on both CPU and GPU,
with a handwritten CUDA backend.

Supported input formats:
- `Chess768`, chess board of features `(colour, piece, square)`.
- `HalfKA`, chess board of features `(king square, colour, piece, square)`

To learn how it works, read the [wiki](wiki.md).

## Usage

### Data

The trainer uses its own binary data format.

You can convert a [Marlinformat](https://github.com/jnlt3/marlinflow) file by running
```
cargo r -r --bin convertmf <input file path> <output file path>
```
it is up to the user to provide a valid Marlinformat file, as well as shuffling the data beforehand.

Additionally, you can convert legacy text format as in Marlinflow, where
- each line is of the form `<FEN> | <score> | <result>`
- `score` is white relative and in centipawns
- `result` is white relative and of the form `1.0` for win, `0.5` for draw, `0.0` for loss

by using the command
```
cargo r -r --bin convert <input file path> <output file path>
```

### Training

General architecture settings, that must be known at compile time, are found in [`common/src/lib.rs`](common/src/lib.rs).
It is like this because of Rust's limitations when it comes to const code.

After settings those as you please, you can run the trainer using the `run.py` script, and use
```
python3 run.py --help
```
to get a full description of all options.

A sample usage is
```
python3 run.py         \
  --data-path data.bin \
  --test-id net        \
  --threads 6          \
  --lr 0.001           \
  --wdl 0.5            \
  --max-epochs 40      \
  --batch-size 16384   \
  --save-rate 10       \
  --skip-prop 0.0      \
  --lr-step 15         \
  --lr-gamma 0.1
```

of these options, only `data-path`, `threads` and `lr-step` are not default values.

#### Learning Rate Scheduler
There are 3 separate learning rate options:
- `lr-step N` drops the learning rate every `N` epochs by a factor of `lr-gamma`
- `lr-drop N` drops the learning rate once, at `N` epochs, by a factor of `lr-gamma`
- `lr-end x` is exponential LR, starting at `lr` and ending at `x` when at `max-epochs`,
it is equivalent to `lr-step 1` with an appropriate `lr-gamma`.

By default `lr-gamma` is set to 0.1, but no learning rate scheduler is chosen. It is highly
recommended to have at least one learning rate drop during training.
#### CUDA

Add `--cuda` to use CUDA, it will fail to compile if not available. It is not recommended to use CUDA
for small net sizes (unbucketed + hidden layer < 256).

#### Resuming

Every `save-rate` epochs and at the end of training, a quantised network is saved to `/nets`, and a checkpoint
is saved to `/checkpoints` (which contains the raw network params, if you want them). You can "resume" from a checkpoint by
adding `--resume checkpoints/<name of checkpoint folder>` to the run command. This is designed such that if you use an identical
command with the resuming appended, it would be as if you never stopped training, so if using a different command be wary that
it will try to resume from the epoch number the checkpoint was saved at, meaning it will fast-forward Learning Rate to that epoch.
