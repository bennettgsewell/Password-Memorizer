using System;
using System.IO;
using System.Threading;

namespace PasswordMemorizer
{
    class Program
    {
        private static string password;

        const int WAIT_TIME = 1500;

        /// <summary>
        /// Prints a message and waits 5 seconds.
        /// </summary>
        static void PrintMessage(string message, int wait = 5000)
        {
            Console.WriteLine(message);
            Thread.Sleep(wait);
        }

        static void Main(string[] args)
        {
            bool running = true;
            while (running)
            {
                Console.WriteLine("Type filename of password.");
                string filename = Console.ReadLine();

                if (filename == "exit")
                    return;

                //Creates password.txt
                if (!File.Exists(filename))
                {
                    File.Create(filename);
                    PrintMessage(string.Format("Created {0}, please put in the password.", filename));
                    return;
                }

                //Read password.txt
                StreamReader stream = File.OpenText(filename);
                password = stream.ReadLine();
                if (string.IsNullOrEmpty(password))
                {
                    PrintMessage(string.Format("{0}, cannot be an empty file.", filename));
                    return;
                }

                //Test password memory.
                Console.WriteLine("Try your best!");

                bool testing = true;

                uint correctEntries = 0u;

                while (testing)
                {
                    //Console.Write("Play again? [y/n] ");
                    //Char playAgain = Console.ReadKey().KeyChar;
                    //testing = playAgain != 'n';
                    Console.Clear();
                    if (correctEntries == 3u)
                        break;
                    if (correctEntries > 0u)
                        Console.WriteLine($"Correctly entered {correctEntries}/3");
                    if (testing)
                    {
                        if (PasswordTest())
                            correctEntries++;
                        else
                            correctEntries = 0u;
                    }
                }
            }
        }

        /// <summary>
        /// Tests users memory of the password.
        /// </summary>
        static bool PasswordTest()
        {
            int length = password.Length;
            char inputChar, sampleChar;
            bool correct = true;

            for (int i = 0; i < length; i++)
            {
                sampleChar = password[i];
                inputChar = Console.ReadKey().KeyChar;
                Console.CursorLeft -= 1;

                correct &= inputChar == sampleChar;
                if (correct)
                {
                    Console.ForegroundColor = ConsoleColor.Green;
                    Console.Write(sampleChar);
                    if (i + 1 == length)
                    {
                        Console.WriteLine();
                        Console.WriteLine("Great work!!!");
                        Thread.Sleep(WAIT_TIME);
                    }
                    continue;
                }
                else
                {
                    Console.ForegroundColor = ConsoleColor.Red;
                    for (; i < length; i++)
                        Console.Write(password[i]);
                    Console.WriteLine();
                    Console.WriteLine("Failed, try again...");
                    Thread.Sleep(WAIT_TIME);
                    break;
                }
            }

            Console.ForegroundColor = ConsoleColor.White;
            return correct;
        }
    }
}
