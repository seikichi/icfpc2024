require 'open3'

# stdout と stderr を返り値にする
def run_script(script, input=nil)
    output = nil
    error = nil

    Open3.popen3("#{script}") do |stdin, stdout, stderr, thread|
        stdin.puts input if input
        stdin.close

        output = stdout.read
        error = stderr.read
    end

    return output, error
end

def submit_one(problem, testid, ai="Simple")
    # テストケースの名前を表示する
    puts "problem=#{problem} testid=#{testid} ai=#{ai} Submitting..."

    # 実行ファイルを実行する
    output, error = run_script("timeout 10 cargo run -q --manifest-path ../solver/Cargo.toml --bin #{problem} -- --ai #{ai} --input ../courses/#{problem}/#{testid}")
    
    # エラーが発生した場合はエラーを表示して終了する
    if !error.empty?
        puts error
        return
    end

    # outputが空の場合もエラーとして終了する
    if output.empty?
        puts "No output: problem=#{problem} testid=#{testid} ai=#{ai}"
        return
    end

    # 結果を表示する
    # puts output

    # 結果をエンコードする
    encoded = run_script('ruby encode-string.rb', "solve #{testid} #{output}")

    # puts encoded

    # communicate.rb に出力を渡して、結果を取得する
    response = run_script('ruby communicate.rb', encoded)

    # puts response

    # 結果をデコードする
    decoded = run_script('ruby decode-string.rb', response)

    puts decoded
end

# # 試し
# submit_one("lambdaman", "lambdaman4")
# puts "waiting 5 seconds..."
# sleep 5
# submit_one("spaceship", "spaceship2")

# courses直下にあるディレクトリを全て取得
Dir.glob('../courses/*').each do |problem|
    # ディレクトリの名前を取得
    problem = problem.split('/')[-1]

    # lambdamanはいったんスキップ
    next if problem == "lambdaman"

    # ディレクトリ内のファイルを全て取得
    Dir.glob("../courses/#{problem}/*").each do |testid|
        # ディレクトリの場合はスキップ
        next if File.directory?(testid)

        # ファイルの名前を取得
        testid = testid.split('/')[-1]

        # テストケースを提出する
        submit_one(problem, testid)

        # 4秒待つ
        puts "waiting 4 seconds..."
        sleep 4
    end
end